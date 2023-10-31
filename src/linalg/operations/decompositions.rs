use crate::{
    core::prelude::*,
    errors::prelude::*,
    linalg::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Array Linalg Decompositions functions
pub trait ArrayLinalgDecompositions<N: NumericOps> where Self: Sized + Clone {

    /// Compute the qr factorization of a matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let array = Array::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9.], vec![3, 3]);
    /// let result = array.qr().unwrap();
    /// let (q, r) = &result.clone()[0];
    ///
    /// assert_eq!(q, &Array::new(vec![0.12309149097933272, 0.9045340337332908, 0.1111111111111111, 0.4923659639173309, 0.30151134457776335, 0.4444444444444444, 0.8616404368553291, -0.30151134457776435, 0.8888888888888888], vec![3, 3]).unwrap());
    /// assert_eq!(r, &Array::new(vec![8.12403840463596, 9.601136296387953, 11.078234188139945, -6.661338147750939e-15, 0.9045340337332832, 1.809068067466574, 8.11111111111111, 9.555555555555555, 11.0], vec![3, 3]).unwrap());
    /// ```
    fn qr(&self) -> LinalgResult<N>;
}

impl <N: NumericOps> ArrayLinalgDecompositions<N> for Array<N> {

    fn qr(&self) -> LinalgResult<N> {
        self.is_dim_unsupported(&[0, 1])?;
        self.is_square()?;
        if self.ndim()? == 2 {
            Ok(vec![Self::gram_schmidt(self)?])
        } else {
            let shape = self.get_shape()?;
            let sub_shape = shape[self.ndim()? - 2 ..].to_vec();
            let qrs = self
                .ravel()?
                .split(self.len()? / sub_shape.iter().product::<usize>(), None)?
                .iter()
                .map(|arr| arr.reshape(&sub_shape).qr())
                .collect::<Vec<Result<Vec<(Array<N>, Array<N>)>, _>>>()
                .has_error()?.into_iter()
                .flat_map(Result::unwrap)
                .collect::<Vec<(Array<N>, Array<N>)>>();
            Ok(qrs)
        }
    }
}

impl <N: NumericOps> ArrayLinalgDecompositions<N> for Result<Array<N>, ArrayError> {

    fn qr(&self) -> LinalgResult<N> {
        self.clone()?.qr()
    }
}

trait QRHelper<N: NumericOps> {

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
                .map(|u| u.to_f64().powf(2.))
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
}

impl <N: NumericOps> QRHelper<N> for Array<N> {}
