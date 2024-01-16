use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    linalg::prelude::*,
    math::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// `ArrayTrait` - Array Linalg Decompositions functions
pub trait ArrayLinalgDecompositions<N: NumericOps> where Self: Sized + Clone {

    /// Compute the qr factorization of a matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let array = Array::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9.], vec![3, 3]);
    /// let qr = array.qr().unwrap();
    ///
    /// let expected_q = Array::new(vec![0.12309149097933272, 0.9045340337332908, 0.1111111111111111, 0.4923659639173309, 0.30151134457776335, 0.4444444444444444, 0.8616404368553291, -0.30151134457776435, 0.8888888888888888], vec![3, 3]).unwrap();
    /// let expected_r = Array::new(vec![8.12403840463596, 9.601136296387953, 11.078234188139945, -6.494804694057166e-15, 0.9045340337332837, 1.809068067466573, 8.11111111111111, 9.555555555555555, 11.], vec![3, 3]).unwrap();
    ///
    /// assert_eq!(expected_q, qr.q);
    /// assert_eq!(expected_r, qr.r);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn qr(&self) -> QrResult<N>;

    /// Compute the singular value decomposition of a matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let array = Array::new(vec![3., 0., 4., 5.], vec![2, 2]);
    /// let svd = array.svd().unwrap();
    ///
    /// let expected_u = Array::new(vec![0.31795579946111835, 0.9055385138137416, 0.9538673983833564, 2.7166155414412216], vec![2, 2]).unwrap();
    /// let expected_s = Array::flat(vec![6.6717460324828926, 2.342606428329091]).unwrap();
    /// let expected_vt = Array::new(vec![0.7071067811865475, 0.7071067811865475, 0.7071067811865492, 0.7071067811865459], vec![2, 2]).unwrap();
    ///
    /// assert_eq!(expected_u, svd.u);
    /// assert_eq!(expected_s, svd.s);
    /// assert_eq!(expected_vt, svd.vt);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn svd(&self) -> SvdResult<N>;
}

impl <N: NumericOps + Floating> ArrayLinalgDecompositions<N> for Array<N> {

    fn qr(&self) -> QrResult<N> {
        self.is_dim_unsupported(&[0, 1])?;
        self.is_square()?;
        if self.ndim()? == 2 {
            let result = Self::gram_schmidt(self)?;
            Ok(QrData {
                q: result.0,
                r: result.1,
            })
        } else {
            let shape = self.get_shape()?;
            let sub_shape = shape[self.ndim()? - 2 ..].to_vec();
            let qrs = self
                .ravel()?
                .split(self.len()? / sub_shape.iter().product::<usize>(), None)?
                .iter()
                .map(|arr| arr.reshape(&sub_shape).qr())
                .collect::<Vec<Result<QrData<N>, _>>>()
                .has_error()?.into_iter()
                .map(Result::unwrap)
                .collect::<Vec<QrData<N>>>();

            let q = qrs.iter()
                .flat_map(|item| item.clone().q)
                .collect::<Self>()
                .reshape(&self.get_shape()?)?;
            let r = qrs.iter()
                .flat_map(|item| item.clone().r)
                .collect::<Self>()
                .reshape(&self.get_shape()?)?;

            Ok(QrData { q, r })
        }
    }

    fn svd(&self) -> SvdResult<N> {
        self.is_dim_unsupported(&[0, 1])?;
        self.is_square()?;

        if self.ndim()? == 2 {
            if Self::is_identity(self)? {
                return Ok(SvdData {
                    u: self.clone(),
                    s: Self::diagonal(self)?,
                    vt: self.clone(),
                })
            }

            let ata = self
                .transpose(None)
                .dot(self)?;
            let eigen_result = ata.eig()?;

            let eigen = eigen_result.values
                .get_elements()?
                .into_iter()
                .zip(eigen_result.vectors.get_rows()?)
                .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .rev()
                .collect::<Vec<(N, Self)>>();
            let values = eigen.clone()
                .into_iter()
                .map(|i| i.0)
                .collect::<Self>();
            let vectors = eigen
                .into_iter()
                .flat_map(|i| i.1)
                .collect::<Self>()
                .reshape(&eigen_result.vectors.get_shape()?)?;

            let vt = vectors.transpose(None)?;
            let s = values.abs().sqrt()?;
            let u = self
                .dot(&vt)
                .dot(&s.diag(None).inv()?)?;
            Ok(SvdData { u, s, vt })
        } else {
            todo!()
        }
    }
}

impl <N: NumericOps + Floating> ArrayLinalgDecompositions<N> for Result<Array<N>, ArrayError> {

    fn qr(&self) -> QrResult<N> {
        self.clone()?.qr()
    }

    fn svd(&self) -> SvdResult<N> {
        self.clone()?.svd()
    }
}

trait DecompHelper<N: NumericOps + Floating> {

    fn gram_schmidt(arr: &Array<N>) -> Result<(Array<N>, Array<N>), ArrayError> {

        fn project<N: NumericOps>(u: &Array<N>, a: &Array<N>) -> Result<Array<N>, ArrayError> {
            let cols = u.len()?;
            let result = u.inner(a).broadcast_to(vec![cols])?
                / u.inner(u).broadcast_to(vec![cols])?
                * u.clone();
            Ok(result)
        }

        fn normalize<N: NumericOps>(arr: &Array<N>) -> Result<Array<N>, ArrayError> {
            let norm = arr
                .get_elements()?
                .into_iter()
                .map(|u| u.to_f64().powi(2))
                .sum::<f64>().sqrt();
            Array::single(N::from(norm))
                .broadcast_to(vec![arr.len()?])
        }

        let (mut u_vecs, mut e_vecs) = (vec![], vec![]);
        for col in arr.get_columns()? {
            let mut a = col;
            for u in &u_vecs { a -= project(u, &a)? }
            u_vecs.push(a.clone());
            let a_norm = normalize(&a)?;
            e_vecs.push(a / a_norm);
        }

        let q = Array::concatenate(e_vecs, None)
            .reshape(&arr.get_shape()?)
            .transpose(None)?;
        let r = q
            .transpose(None)
            .dot(arr)?;

        Ok((q, r))
    }

    fn is_identity(arr: &Array<N>) -> Result<bool, ArrayError> {
        arr.is_square()?;

        for i in 0..arr.get_rows()?.len() {
            for j in 0..arr.get_shape()?.len() {
                let element = arr.at(&[i, j])?;
                if i == j {
                    if element != N::one() {
                        return Ok(false)
                    }
                } else if element != N::zero() {
                    return Ok(false)
                }
            }
        }

        Ok(true)
    }

    fn diagonal(arr: &Array<N>) -> Result<Array<N>, ArrayError> {
        arr.is_square()?;

        let result = (0..arr.get_rows()?.len())
            .map(|i| arr.at(&[i, i]).unwrap())
            .collect();
        Ok(result)
    }
}

impl <N: NumericOps + Floating> DecompHelper<N> for Array<N> {}
