use crate::{
    core::prelude::*,
    errors::prelude::*,
    linalg::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// `ArrayTrait` - Array Linalg Eigen functions
pub trait ArrayLinalgEigen<N: NumericOps> where Self: Sized + Clone {

    /// Compute the eigenvalues of a square array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let array = Array::new(vec![12., -51., 4., 6., 167., -68., -4., 24., -41.], vec![3, 3]).unwrap();
    /// let vals = array.eigvals().unwrap();
    /// assert_eq!(Array::flat(vec![156.20350762022625, -35.473244608879945, 17.26973698865371]).unwrap(), vals);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn eigvals(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the eigenvalues and right eigenvectors of a square array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let array = Array::new(vec![12., -51., 4., 6., 167., -68., -4., 24., -41.], vec![3, 3]).unwrap();
    /// let result = array.eig().unwrap();
    /// assert_eq!(Array::flat(vec![156.20350762022625, -35.473244608879945, 17.26973698865371]).unwrap(), result.values);
    /// assert_eq!(Array::new(vec![0.3274744043621118, 0.2741043942532785, -0.9929229419852038, -0.9370666300603048, 0.3040145084128465, 0.08536232642252124, -0.12110592601150529, 0.9123825731158716, 0.08256696983166067], vec![3, 3]).unwrap(), result.vectors);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn eig(&self) -> EigenResult<N>;
}

impl <N: NumericOps + Floating> ArrayLinalgEigen<N> for Array<N> {

    fn eigvals(&self) -> Result<Self, ArrayError> {
        self.is_square()?;
        if self.ndim()? == 2 {
            let mut h = self.hessenberg_reduction()?;

            let mut max_iter = 10000;
            while max_iter > 0 {
                let qr = h.qr()?.clone();
                h = qr.r.dot(&qr.q)?.hessenberg_reduction()?;
                if h.is_convergent()? { break }
                max_iter -= 1;
            }

            let mut eigenvalues = vec![];
            for i in 0..self.get_shape()?[0] {
                eigenvalues.push(h.at(&[i, i])?);
            }

            Ok(Self::flat(eigenvalues)?)
        } else {
            let shape = self.get_shape()?;
            let sub_shape = shape[self.ndim()? - 2 ..].to_vec();
            self
                .ravel()?
                .split(self.len()? / sub_shape.iter().product::<usize>(), None)?
                .iter()
                .map(|arr| arr.reshape(&sub_shape).eigvals())
                .collect::<Vec<Result<Self, _>>>()
                .has_error()?.into_iter()
                .flat_map(Result::unwrap)
                .collect::<Self>()
                .reshape(&shape)
        }
    }

    fn eig(&self) -> EigenResult<N> {
        let mut vectors = vec![];
        let eigenvalues = self.eigvals()?;

        let solve_vec = Self::ones(vec![self.get_shape()?[0]])?;
        for value in eigenvalues.get_elements()? {
            let mut vector = (self.clone() - (Self::eye(self.get_shape()?[0], None, None)? * value)?)
                .solve(&solve_vec)?;
            vector /= vector
                .norm(None::<NormOrd>, None, None)
                .broadcast_to(vector.get_shape()?)?;
            vectors.push(vector);
        }
        let eigenvectors = vectors.iter()
            .flatten()
            .copied()
            .collect::<Self>()
            .reshape(&self.get_shape()?)
            .transpose(None)?;
        Ok(EigenData { values: eigenvalues, vectors: eigenvectors })
    }
}

impl <N: NumericOps + Floating> ArrayLinalgEigen<N> for Result<Array<N>, ArrayError> {

    fn eigvals(&self) -> Self {
        self.clone()?.eigvals()
    }

    fn eig(&self) -> EigenResult<N> {
        self.clone()?.eig()
    }
}
