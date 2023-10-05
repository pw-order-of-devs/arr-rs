use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    math::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Array Linalg Products functions
pub trait ArrayLinalgProducts<N: Numeric> where Self: Sized + Clone {

    /// Dot product of two arrays
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
    /// assert_eq!(Array::single(12), Array::single(3).dot(&Array::single(4).unwrap()));
    /// assert_eq!(Array::single(20), Array::flat(vec![1, 2, 3]).dot(&Array::flat(vec![2, 3, 4]).unwrap()));
    /// assert_eq!(Array::new(vec![4, 1, 2, 2], vec![2, 2]), Array::new(vec![1, 0, 0, 1], vec![2, 2]).dot(&Array::new(vec![4, 1, 2, 2], vec![2, 2]).unwrap()));
    /// ```
    fn dot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Dot product of two vectors. If input is an array, it will be raveled
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
    /// assert_eq!(Array::single(20), Array::flat(vec![1, 2, 3]).vdot(&Array::flat(vec![2, 3, 4]).unwrap()));
    /// assert_eq!(Array::single(30), Array::new(vec![1, 4, 5, 6], vec![2, 2]).vdot(&Array::new(vec![4, 1, 2, 2], vec![2, 2]).unwrap()));
    /// ```
    fn vdot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Matrix product of two arrays
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
    /// assert_eq!(Array::single(5), Array::flat(vec![1, 2]).matmul(&Array::flat(vec![1, 2]).unwrap()));
    /// assert_eq!(Array::new(vec![5, 8, 8, 13], vec![2, 2]), Array::new(vec![1, 2, 2, 3], vec![2, 2]).matmul(&Array::new(vec![1, 2, 2, 3], vec![2, 2]).unwrap()));
    /// ```
    fn matmul(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayLinalgProducts<N> for Array<N> {

    fn dot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        if self.len()? == 1 || other.len()? == 1 {
            self.multiply(other)
        } else if self.ndim()? == 1 && other.ndim()? == 1 {
            self.vdot(other)
        } else if self.ndim()? == 2 && other.ndim()? == 2 {
            self.matmul(other)
        } else if self.ndim()? == 1 || other.ndim()? == 1 {
            Self::dot_1d(self, other)
        } else {
            Self::dot_nd(self, other)
        }
    }

    fn vdot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.len()?.is_equal(&other.len()?)?;
        let result = self.ravel()?.zip(&other.ravel()?)?
            .map(|tuple| tuple.0.to_f64() * tuple.1.to_f64())?
            .fold(0., |a, b| a + b)?;
        Array::single(N::from(result))
    }

    fn matmul(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        if self.ndim()? == 1 && other.ndim()? == 1 {
            self.vdot(other)
        } else if self.ndim()? == 1 || other.ndim()? == 1 {
            if self.ndim()? == 1 { self.shapes_align(0, &other.get_shape()?, other.ndim()? - 1)?; }
            else { self.shapes_align(self.ndim()? - 1, &other.get_shape()?, 0)?; }
            Self::matmul_1d_nd(self, other)
        } else if self.ndim()? == 2 && other.ndim()? == 2 {
            self.shapes_align(0, &other.get_shape()?, 1)?;
            Self::matmul_iterate(self, other)
        } else {
            Self::matmul_nd(self, other)
        }
    }
}

impl <N: Numeric> ArrayLinalgProducts<N> for Result<Array<N>, ArrayError> {

    fn dot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.dot(other)
    }

    fn vdot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.vdot(other)
    }

    fn matmul(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.matmul(other)
    }
}

trait LinalgHelper {

    fn dot_split_array<N: Numeric>(arr: &Array<N>, axis: usize) -> Result<Vec<Array<N>>, ArrayError> {
        arr.split_axis(axis)?
            .into_iter().flatten()
            .collect::<Array<N>>()
            .split(arr.get_shape()?.remove_at(axis).iter().product(), None)
    }

    fn dot_iterate<N: Numeric>(v_arr_1: &[Array<N>], v_arr_2: &[Array<N>]) -> Result<Array<N>, ArrayError> {
        v_arr_1.iter().flat_map(|a| {
            v_arr_2.iter().map(move |b| a.vdot(b))
        })
            .collect::<Vec<Result<Array<N>, _>>>()
            .has_error()?.into_iter()
            .flat_map(|item| item.unwrap())
            .collect::<Array<N>>()
            .ravel()
    }

    fn matmul_iterate<N: Numeric>(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        let (shape_1, shape_2) = (&arr_1.get_shape()?, &arr_2.get_shape()?);
        (0..shape_1[0])
            .flat_map(|i| (0..shape_2[1])
                .map(move |j| (0..shape_1[1])
                    .fold(0., |acc, k| acc + arr_1[i * shape_1[1] + k].to_f64() * arr_2[k * shape_2[1] + j].to_f64())))
            .map(N::from_f64)
            .collect::<Array<N>>()
            .reshape(&[shape_1[0], shape_2[1]])
    }

    fn dot_1d<N: Numeric>(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        let v_arr_1 = Self::dot_split_array(arr_1, arr_1.ndim()? - 1)?;
        let v_arr_2 = Self::dot_split_array(arr_2, 0)?;
        Self::dot_iterate(&v_arr_1, &v_arr_2)
    }

    fn dot_nd<N: Numeric>(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        arr_1.shapes_align(arr_1.ndim()? - 1, &arr_2.get_shape()?, arr_2.ndim()? - 2)?;
        let mut new_shape = arr_1.get_shape()?.remove_at(arr_1.ndim()? - 2);
        new_shape.extend_from_slice(&arr_2.get_shape()?.remove_at(arr_2.ndim()? - 1));
        let v_arr_1 = Self::dot_split_array(arr_1, arr_1.ndim()? - 2)?;
        let v_arr_2 = Self::dot_split_array(arr_2, arr_2.ndim()? - 1)?;

        let rev = arr_2.len()? > arr_1.len()?;
        let pairs = (0..new_shape.len() as isize)
            .collect::<Vec<isize>>()
            .reverse_if(rev)
            .into_iter()
            .step_by(2)
            .map(|item|
                if rev { if item <= 1 { vec![item] } else { vec![item, item - 1] } }
                else if new_shape.len() as isize > item + 1 { vec![item + 1, item] }
                else { vec![item] })
            .collect::<Vec<Vec<isize>>>()
            .reverse_if(rev)
            .into_iter()
            .flatten()
            .collect::<Vec<isize>>();
        Self::dot_iterate(&v_arr_1, &v_arr_2)
            .reshape(&new_shape)
            .transpose(Some(pairs))
    }

    fn matmul_1d_nd<N: Numeric>(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        if arr_1.ndim()? == 1 {
            if arr_2.ndim()? > 2 {
                let new_shape = arr_2.get_shape()?.remove_at(0);
                arr_2.split_axis(0)?.into_iter()
                    .map(|arr| Self::matmul_1d_nd(arr_1, &arr.reshape(&new_shape).unwrap()))
                    .collect::<Vec<Result<Array<N>, _>>>()
                    .has_error()?
                    .into_iter()
                    .flat_map(Result::unwrap)
                    .collect::<Array<N>>()
                    .reshape(&new_shape)
            } else {
                let result = arr_1
                    .get_elements()?
                    .into_iter()
                    .zip(&arr_2.split_axis(0)?)
                    .map(|(a, b)| b.into_iter()
                        .map(|item| a.to_f64() * item.to_f64())
                        .sum::<f64>())
                    .map(N::from)
                    .collect::<Array<N>>();
                Ok(result)
            }
        } else if arr_1.ndim()? > 2 {
            let new_shape = arr_1.get_shape()?.remove_at(0);
            arr_1.split_axis(0)?
                .into_iter()
                .map(|arr| Self::matmul_1d_nd(&arr.reshape(&new_shape).unwrap(), arr_2))
                .collect::<Vec<Result<Array<N>, _>>>()
                .has_error()?
                .into_iter()
                .flat_map(Result::unwrap)
                .collect::<Array<N>>()
                .reshape(&new_shape)
        } else {
            let result = arr_1
                .split_axis(0)?
                .iter()
                .map(|arr| (0..arr.shape[arr.shape.len() - 1])
                    .map(|idx| arr[idx].to_f64() * arr_2[idx].to_f64())
                    .sum::<f64>())
                .map(N::from)
                .collect::<Array<N>>();
            Ok(result)
        }
    }

    fn matmul_nd<N: Numeric>(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        fn matmul_split<N: Numeric>(arr: &Array<N>, len: usize, chunk_len: usize) -> Result<Vec<Array<N>>, ArrayError> {
            let shape_last = (arr.get_shape()?[arr.ndim()? - 2], arr.get_shape()?[arr.ndim()? - 1]);
            let result = arr.split(arr.len()? / chunk_len, Some(0))?
                .into_iter().cycle().take(len)
                .map(|arr| arr.reshape(&[shape_last.0, shape_last.1]).unwrap())
                .collect::<Vec<Array<N>>>();
            Ok(result)
        }

        let mut new_shape =
            if arr_1.ndim()? >= arr_2.ndim()? { arr_1.get_shape()? }
            else { arr_2.get_shape()? };
        let shape_len = new_shape.len();
        new_shape[shape_len - 2] = arr_1.get_shape()?[arr_1.ndim()? - 2];
        new_shape[shape_len - 1] = arr_2.get_shape()?[arr_2.ndim()? - 1];
        let chunk_len = arr_1.get_shape()?[arr_1.ndim()? - 2 ..].iter().product::<usize>();
        let len = std::cmp::max(arr_1.len()?, arr_2.len()?) / chunk_len;
        matmul_split(arr_1, len, chunk_len)?
            .into_iter()
            .zip(&matmul_split(arr_2, len, chunk_len)?)
            .map(|(a, b)| a.matmul(b))
            .collect::<Vec<Result<Array<N>, _>>>()
            .has_error()?
            .into_iter()
            .flat_map(Result::unwrap)
            .collect::<Array<N>>()
            .reshape(&new_shape)
    }
}

impl <N: Numeric> LinalgHelper for Array<N> {}
