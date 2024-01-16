use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    linalg::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// `ArrayTrait` - Array Linalg Solving equations and Inverting matrices functions
pub trait ArrayLinalgSolvingInvertingProducts<N: NumericOps> where Self: Sized + Clone {

    /// Solve a linear matrix equation, or system of linear scalar equations
    ///
    /// # Arguments
    ///
    /// * `other` - other array to perform operations with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr_1 = Array::new(vec![2., 1., 1., 3.], vec![2, 2]).unwrap();
    /// let arr_2 = Array::new(vec![5., 8., 3., 6.], vec![2, 2]).unwrap();
    /// assert_eq!(Array::new(vec![2.4, 3.6, 0.2, 0.8], vec![2, 2]), arr_1.solve(&arr_2));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn solve(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Compute the (multiplicative) inverse of a matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![3., 0., 4., 5.], vec![2, 2]).unwrap();
    /// assert_eq!(Array::new(vec![0.3333333333333333, 0., 0.25, 0.2], vec![2, 2]), arr.inv());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn inv(&self) -> Result<Array<N>, ArrayError>;
}

impl <N: NumericOps> ArrayLinalgSolvingInvertingProducts<N> for Array<N> {

    fn solve(&self, other: &Self) -> Result<Self, ArrayError> {
        self.is_dim_supported(&[2])?;
        let n = self.get_shape()?[0];
        self.is_square()?;
        other.get_shape()?[0].is_equal(&n)?;

        if self.det()?[0].to_f64().abs() < 1e-12 {
            return Err(ArrayError::SingularMatrix);
        };

        let mut arr_l = Self::identity(n)?.to_array_f64()?.to_matrix()?;
        let mut arr_u = self.to_array_f64()?.to_matrix()?;

        for j in 0..n {
            let mut pivot_row = j;
            for i in j + 1..n {
                if arr_u[i][j].abs() > arr_u[pivot_row][j].abs() { pivot_row = i; }
            }

            if pivot_row != j {
                let tmp = arr_u[pivot_row].clone();
                arr_u[pivot_row] = arr_u[j].clone();
                arr_u[j] = tmp;

                let tmp = arr_l[pivot_row].clone();
                for (idx, item) in tmp.iter().enumerate().take(j) {
                    arr_l[pivot_row][idx] = arr_l[j][idx];
                    arr_l[j][idx] = *item;
                }
            }
            for i in j + 1..n {
                let factor = arr_u[i][j] / arr_u[j][j];
                arr_l[i][j] = factor;
                let uu = ((arr_u[j][j..].to_vec().to_array()? * factor)?).get_elements()?;
                for jj in j..n {
                    arr_u[i][jj] -= uu[jj - j];
                }
            }
        }

        let other = other.to_array_f64()?;
        let mut arr_y = Array::<f64>::zeros_like(&other)?.get_rows()?;
        let arr_b = other.to_array_f64()?.get_rows()?;
        for i in 0..n {
            let l_tmp = arr_l[i][..i].to_vec().to_array()?;
            let y_tmp = arr_y[..i].iter().flatten().copied().collect::<Vec<f64>>().to_array()?;
            let dot = l_tmp.dot(&y_tmp).unwrap_or(Array::flat(vec![0.; arr_b[0].len()?])?);
            arr_y[i] = arr_b[i].broadcast_to(dot.get_shape()?)? - dot;
        }

        let mut arr_x = Array::<f64>::zeros_like(&other)?.get_rows()?;
        for i in (0..n).rev() {
            let u_tmp = arr_u[i][i + 1..].to_vec().to_array()?;
            let x_tmp = arr_x[i + 1..].iter().flatten().copied().collect::<Vec<f64>>().to_array()?;
            let dot = u_tmp.dot(&x_tmp).unwrap_or(Array::flat(vec![0.; arr_b[0].len()?])?);
            arr_x[i] = ((arr_y[i].clone() - dot) / arr_u[i][i])?;
        }

        arr_x.into_iter()
            .flatten()
            .collect::<Array<_>>()
            .to_array_num()
            .reshape(&other.get_shape()?)
    }

    fn inv(&self) -> Result<Self, ArrayError> {
        self.map(|&x| if x == N::zero() { N::zero() } else { N::one() / x })
    }
}

impl <N: NumericOps> ArrayLinalgSolvingInvertingProducts<N> for Result<Array<N>, ArrayError> {

    fn solve(&self, other: &Array<N>) -> Self {
        self.clone()?.solve(other)
    }

    fn inv(&self) -> Self {
        self.clone()?.inv()
    }
}
