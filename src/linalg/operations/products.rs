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
}

impl <N: Numeric> ArrayLinalgProducts<N> for Array<N> {

    fn dot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        if self.len()? == 1 && other.len()? == 1 {
            Array::single(N::from(self[0].to_f64() * other[0].to_f64()))
        } else if other.len()? == 1 {
            self.multiply(other)
        } else if self.len()? == 1 {
            other.multiply(self)
        } else if self.ndim()? == 1 && other.ndim()? == 1 {
            Self::dot_sum_product(self, other)
        } else if self.ndim()? == 2 && other.ndim()? == 2 {
            Self::dot_mat_mul(self, other)
        } else if self.ndim()? == 1 || other.ndim()? == 1 {
            Self::dot_1d(self, other)
        } else {
            Self::dot_nd(self, other)
        }
    }
}

#[test] fn test() {
    let arr_1 = Array::new(vec![1, 2, 3, 4], vec![2, 2]).unwrap();
    let arr_2 = Array::flat(vec![1, 2]).unwrap();
    println!("{}", arr_1.dot(&arr_2).unwrap());

    let arr_1 = Array::flat(vec![1, 2]).unwrap();
    let arr_2 = Array::new(vec![1, 2, 3, 4], vec![2, 2]).unwrap();
    println!("{}", arr_1.dot(&arr_2).unwrap());

    let arr_1 = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]).unwrap();
    let arr_2 = Array::new(vec![1, 2, 3, 4], vec![2, 2]).unwrap();
    println!("{}", arr_1.dot(&arr_2).unwrap());

    let arr_1 = Array::new(vec![1, 2, 3, 4], vec![2, 2]).unwrap();
    let arr_2 = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]).unwrap();
    println!("{}", arr_1.dot(&arr_2).unwrap());
}

impl <N: Numeric> ArrayLinalgProducts<N> for Result<Array<N>, ArrayError> {

    fn dot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.dot(other)
    }
}

trait DotHelper {

    fn dot_sum_product<N: Numeric>(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        if arr_1.len()? != arr_2.len()? {
            return Err(ArrayError::ParameterError { param: "`len`", message: "of inputs for sum_product must be equal" })
        }
        let result = arr_1.zip(arr_2)?
            .map(|tuple| tuple.0.to_f64() * tuple.1.to_f64())?
            .fold(0., |a, b| a + b)?;
        Array::single(N::from(result))
    }

    fn dot_mat_mul<N: Numeric>(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        if arr_1.get_shape()? != arr_2.get_shape()? {
            return Err(ArrayError::ShapesMustMatch { shape_1: arr_1.get_shape()?, shape_2: arr_2.get_shape()? })
        }
        let mut result = vec![];
        let (n, m) = (arr_1.get_shape()?[0], arr_1.get_shape()?[1]);
        for i in 0 .. n { for j in 0 .. m {
            let sum = (0 .. n).fold(0., |a, k| a + arr_1[i * n + k].to_f64() * arr_2[k * m + j].to_f64());
            result.push(N::from(sum));
        } }
        Array::new(result, arr_1.get_shape()?)
    }

    fn dot_split_array<N: Numeric>(arr: &Array<N>, axis: usize) -> Result<Vec<Array<N>>, ArrayError> {
        arr.split_axis(axis)?
            .into_iter().flatten()
            .collect::<Array<N>>()
            .split(arr.get_shape()?.remove_at(axis).iter().product(), None)
    }

    fn dot_iterate<N: Numeric>(v_arr_1: &[Array<N>], v_arr_2: &[Array<N>]) -> Result<Array<N>, ArrayError> {
        v_arr_1.iter().flat_map(|a| {
            v_arr_2.iter().map(move |b| Self::dot_sum_product(a, b))
        })
            .collect::<Vec<Result<Array<N>, _>>>()
            .has_error()?.into_iter()
            .flat_map(|item| item.unwrap())
            .collect::<Array<N>>()
            .ravel()
    }

    fn dot_1d<N: Numeric>(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        let v_arr_1 = Self::dot_split_array(arr_1, arr_1.ndim()? - 1)?;
        let v_arr_2 = Self::dot_split_array(arr_2, 0)?;
        Self::dot_iterate(&v_arr_1, &v_arr_2)
    }

    fn dot_nd<N: Numeric>(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        if arr_1.get_shape()?[arr_1.ndim()? - 1] != arr_2.get_shape()?[arr_2.ndim()? - 2] {
            return Err(ArrayError::ParameterError { param: "`shapes`", message: "are not aligned" })
        }
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
}

impl <N: Numeric> DotHelper for Array<N> {}
