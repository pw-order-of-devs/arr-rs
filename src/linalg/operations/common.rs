use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    linalg::prelude::*,
    math::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

pub(crate) trait LinalgHelper<N: NumericOps> {

    fn get_columns(&self) -> Result<Vec<Self>, ArrayError> where Self: Sized;

    fn get_rows(&self) -> Result<Vec<Self>, ArrayError> where Self: Sized;

    fn to_matrix(&self) -> Result<Vec<Vec<N>>, ArrayError>;

    fn from_matrix(matrix: &[Vec<N>]) -> Result<Self, ArrayError> where Self: Sized;

    fn is_convergent(&self) -> Result<bool, ArrayError>;

    fn hessenberg_reduction(&self) -> Result<Self, ArrayError> where Self: Sized;
}

impl <N: NumericOps> LinalgHelper<N> for Array<N> {

    fn get_columns(&self) -> Result<Vec<Self>, ArrayError> {
        let col_len = self.get_shape()?[0];
        let cols = self.get_shape()?[1];
        let elements = self
            .transpose(None)
            .get_elements()?;
        let result = parse_elements(&elements, cols, col_len);
        Ok(result)
    }

    fn get_rows(&self) -> Result<Vec<Self>, ArrayError> where Self: Sized {
        if self.ndim()? == 1 {
            return self.split(self.len()?, None);
        }

        let row_len = self.get_shape()?[1];
        let rows = self.get_shape()?[0];
        let elements = self
            .get_elements()?;
        let result = parse_elements(&elements, rows, row_len);
        Ok(result)
    }

    fn to_matrix(&self) -> Result<Vec<Vec<N>>, ArrayError> {
        self.is_dim_supported(&[2])?;
        let matrix = self
            .get_elements()?
            .chunks(self.get_shape()?[1])
            .map(<[N]>::to_vec)
            .collect();
        Ok(matrix)
    }

    fn from_matrix(matrix: &[Vec<N>]) -> Result<Self, ArrayError> {
        let (r, c) = (matrix.len(), matrix[0].len());
        let array = matrix.iter()
            .flatten()
            .copied()
            .collect::<Vec<N>>();
        Self::new(array, vec![r, c])
    }

    fn is_convergent(&self) -> Result<bool, ArrayError> {
        self.is_dim_supported(&[2])?;
        let matrix = self.to_array_f64()?.to_matrix()?;

        for (i, item) in matrix.iter().enumerate() {
            let diagonal = item[i].to_f64().abs();
            let row_sum = item.iter().map(|e| e.abs()).sum::<f64>() - diagonal;


            if diagonal <= row_sum {
                return Ok(false)
            }
        }

        Ok(true)
    }

    fn hessenberg_reduction(&self) -> Result<Self, ArrayError> {
        let n = self.get_shape()?[0];
        let mut h = self.clone();

        for k in 0..n - 2 {
            let column = h.get_columns()?[k].clone();
            let x = column.into_iter().skip(k + 1).take(n - (k + 1)).collect();
            let mut v = Self::zeros_like(&x)?;
            let x_norm = x.norm(None::<NormOrd>, None, None)?[0];
            v[0] = N::from(x.sign()?[0]) * x_norm;
            v = x.clone() - v;
            let v_norm = v.norm(None::<NormOrd>, None, None)?[0];
            if v_norm != N::zero() { v /= v_norm; }

            let mut hh = Self::concatenate(h.get_rows()?[k + 1..].to_vec(), None)
                .reshape(&[n - k - 1, n])?;
            hh = Self::concatenate(hh.get_columns()?[k..].to_vec(), None)?
                .reshape(&[n - k, n - k - 1])
                .transpose(None)?;
            let outer = v.outer(&v.dot(&hh)?)?;
            hh -= (outer * N::from(2))?;
            let mut h_m = h.to_matrix()?;
            let hh_m = hh.to_matrix()?;
            for i in 0..hh_m.len() { for j in 0..hh_m[0].len() {
                h_m[i + k + 1][j + k] = hh_m[i][j];
            } }
            h = Self::from_matrix(&h_m)?;

            let mut hh = Self::concatenate(h.get_columns()?[k + 1..].to_vec(), None)
                .reshape(&[n - k - 1, n])
                .transpose(None)?;
            let outer = hh.dot(&v).outer(&v)?;
            hh -= (outer * N::from(2))?;
            let mut h_m = h.to_matrix()?;
            let hh_m = hh.to_matrix()?;
            for i in 0..hh_m.len() { for j in 0..hh_m[0].len() {
                h_m[i][j + k + 1] = hh_m[i][j];
            } }
            h = Self::from_matrix(&h_m)?;
        }

        Ok(h)
    }
}

fn parse_elements<N: NumericOps>(elements: &[N], count_1: usize, count_2: usize) -> Vec<Array<N>> {
    (0..count_1)
        .map(|i| elements
            .to_vec()
            .iter()
            .copied()
            .skip(i * count_2)
            .take(count_2)
            .collect())
        .collect()
}
