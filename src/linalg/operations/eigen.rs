use crate::{
    core::prelude::*,
    errors::prelude::*,
    linalg::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Array Linalg Eigen functions
pub trait ArrayLinalgEigen<N: NumericOps> where Self: Sized + Clone {

    /// Compute the eigenvalues of a square array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let array = Array::new(vec![12., -51., 4., 6., 167., -68., -4., 24., -41.], vec![3, 3]).unwrap();
    /// let vals = array.eigvals().unwrap()[0].clone();
    /// assert_eq!(Array::flat(vec![156.20350762022628, -35.47324460887994, 17.269736988653587]).unwrap(), vals);
    /// ```
    fn eigvals(&self) -> Result<Vec<Array<N>>, ArrayError>;

    /// Compute the eigenvalues and right eigenvectors of a square array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let array = Array::new(vec![12., -51., 4., 6., 167., -68., -4., 24., -41.], vec![3, 3]).unwrap();
    /// let (vals, vecs) = array.eig().unwrap()[0].clone();
    /// assert_eq!(Array::flat(vec![156.20350762022628, -35.47324460887994, 17.269736988653587]).unwrap(), vals);
    /// assert_eq!(Array::new(vec![0.3274744043621115, 0.27410439425327837, -0.9929229419852036, -0.9370666300603048, 0.30401450841284644, 0.08536232642252142, -0.12110592601150547, 0.9123825731158715, 0.08256696983166305], vec![3, 3]).unwrap(), vecs);
    /// ```
    fn eig(&self) -> LinalgResult<N>;
}

impl <N: NumericOps> ArrayLinalgEigen<N> for Array<N> {

    fn eigvals(&self) -> Result<Vec<Array<N>>, ArrayError> {
        self.is_square()?;
        if self.ndim()? == 2 {
            let mut h = self.hessenberg_reduction()?;

            let mut max_iter = 10000;
            // fix condition - check convergence
            while max_iter > 0 {
                let (q, r) = h.qr()?[0].clone();
                h = r.dot(&q)?.hessenberg_reduction()?;
                if h.is_convergent()? { break }
                max_iter -= 1;
            }

            let mut eigenvalues = vec![];
            for i in 0 .. self.get_shape()?[0] {
                eigenvalues.push(h.at(&[i, i])?);
            }

            Ok(vec![Array::flat(eigenvalues)?])
        } else {
            let shape = self.get_shape()?;
            let sub_shape = shape[self.ndim()? - 2 ..].to_vec();
            let vals = self
                .ravel()?
                .split(self.len()? / sub_shape.iter().product::<usize>(), None)?
                .iter()
                .map(|arr| arr.reshape(&sub_shape).eigvals())
                .collect::<Vec<Result<Vec<Array<N>>, _>>>()
                .has_error()?.into_iter()
                .flat_map(Result::unwrap)
                .collect::<Vec<Array<N>>>();
            Ok(vals)
        }
    }

    fn eig(&self) -> LinalgResult<N> {
        let mut results = vec![];
        for eigenvalues in self.eigvals()? {
            let mut vectors = vec![];
            for value in eigenvalues.get_elements()? {
                let mut vector = (self.clone() - (Array::eye(self.get_shape()?[0], None, None)? * value)?)
                    .solve(&Array::ones(vec![self.get_shape()?[0]])?)?;
                vector /= vector
                    .norm(None::<NormOrd>, None, None)
                    .broadcast_to(vector.get_shape()?)?;
                vectors.push(vector);
            }
            let eigenvectors = vectors.iter()
                .flatten()
                .cloned()
                .collect::<Self>()
                .reshape(&[eigenvalues.len()?; 2])
                .transpose(None)?;
            results.push((eigenvalues, eigenvectors));
        }

        Ok(results)
    }
}

impl <N: NumericOps> ArrayLinalgEigen<N> for Result<Array<N>, ArrayError> {

    fn eigvals(&self) -> Result<Vec<Array<N>>, ArrayError> {
        self.clone()?.eigvals()
    }

    fn eig(&self) -> LinalgResult<N> {
        self.clone()?.eig()
    }
}
