use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    linalg::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Array Linalg Solving equations and Inverting matrices functions
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
    fn solve(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;
}

impl <N: NumericOps> ArrayLinalgSolvingInvertingProducts<N> for Array<N> {

    fn solve(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.is_dim_unsupported(&[0, 1])?;
        if self.ndim()? == 2 {
            let n = self.get_shape()?[0];
            self.is_square()?;
            other.get_shape()?[0].is_equal(&n)?;

            if self.det()?[0].to_f64().abs() < 1e-12 {
                return Err(ArrayError::SingularMatrix);
            };

            let mut l = Self::identity(n)?.to_array_f64()?.to_matrix()?;
            let mut u = self.to_array_f64()?.to_matrix()?;

            for j in 0 .. n {
                let mut pivot_row = j;
                for i in j + 1 .. n {
                    if u[i][j].abs() > u[pivot_row][j].abs() { pivot_row = i; }
                }

                if pivot_row != j {
                    let tmp = u[pivot_row].clone();
                    u[pivot_row] = u[j].clone();
                    u[j] = tmp;

                    let tmp = l[pivot_row].clone();
                    for (idx, item) in tmp.iter().enumerate().take(j) {
                        l[pivot_row][idx] = l[j][idx];
                        l[j][idx] = *item;
                    }
                }
                for i in j + 1 .. n {
                    let factor = u[i][j] / u[j][j];
                    l[i][j] = factor;
                    let uu = ((u[j][j..].to_vec().to_array()? * factor)?).get_elements()?;
                    for jj in j .. n {
                        u[i][jj] -= uu[jj - j];
                    }
                }
            }

            let other = other.to_array_f64()?;
            let mut y = Array::<f64>::zeros_like(&other.clone())?.get_rows()?;
            let b = other.to_array_f64()?.get_rows()?;
            for i in 0 .. n {
                let l_tmp = l[i][..i].to_vec().to_array()?;
                let y_tmp = y[..i].iter().flatten().cloned().collect::<Vec<f64>>().to_array()?;
                let dot = l_tmp.dot(&y_tmp).unwrap_or(Array::flat(vec![0.; b[0].len()?])?);
                y[i] = b[i].broadcast_to(dot.get_shape()?)? - dot;
            }

            let mut x = Array::<f64>::zeros_like(&other.clone())?.get_rows()?;
            for i in (0 ..= n - 1).rev() {
                let u_tmp = u[i][i + 1..].to_vec().to_array()?;
                let x_tmp = x[i + 1..].iter().flatten().cloned().collect::<Vec<f64>>().to_array()?;
                let dot = u_tmp.dot(&x_tmp).unwrap_or(Array::flat(vec![0.; b[0].len()?])?);
                x[i] = ((y[i].clone() - dot) / u[i][i])?;
            }

            x.into_iter()
                .flatten()
                .collect::<Array<_>>()
                .to_array_num()
                .reshape(&other.get_shape()?)
        } else {
            todo!()
        }
    }
}

impl <N: NumericOps> ArrayLinalgSolvingInvertingProducts<N> for Result<Array<N>, ArrayError> {

    fn solve(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.solve(other)
    }
}
