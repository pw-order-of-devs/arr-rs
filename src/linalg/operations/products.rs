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

    /// Inner product of two arrays
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
    /// assert_eq!(Array::single(20), Array::flat(vec![1, 2, 3, 4]).inner(&Array::flat(vec![4, 3, 2, 1]).unwrap()));
    /// assert_eq!(Array::new(vec![10, 4, 24, 10], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]).inner(&Array::new(vec![4, 3, 2, 1], vec![2, 2]).unwrap()));
    /// ```
    fn inner(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Outer product of two arrays
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
    /// assert_eq!(Array::new(vec![4, 3, 8, 6], vec![2, 2]), Array::flat(vec![1, 2]).outer(&Array::flat(vec![4, 3]).unwrap()));
    /// assert_eq!(Array::new(vec![4, 3, 2, 1, 8, 6, 4, 2, 12, 9, 6, 3, 16, 12, 8, 4], vec![4, 4]), Array::new(vec![1, 2, 3, 4], vec![2, 2]).outer(&Array::new(vec![4, 3, 2, 1], vec![2, 2]).unwrap()));
    /// ```
    fn outer(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

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

    /// Compute the determinant of an array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(Array::single(-14), Array::new(vec![3, 8, 4, 6], vec![2, 2]).det());
    /// ```
    fn det(&self) -> Result<Array<N>, ArrayError>;
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

    fn inner(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        if self.ndim()? == 1 && other.ndim()? == 1 {
            self.shapes_align(0, &other.get_shape()?, 0)?;
            self.zip(other)?
                .map(|i| i.0.to_f64() * i.1.to_f64())
                .sum(None)?
                .to_array_num()
        } else {
            self.shapes_align(self.ndim()? - 1, &other.get_shape()?, other.ndim()? - 1)?;
            Self::inner_nd(self, other)
        }
    }

    fn outer(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.into_iter().flat_map(|a| other.into_iter()
            .map(|b| N::from(a.to_f64() * b.to_f64()))
            .collect::<Array<N>>())
            .collect::<Array<N>>()
            .reshape(&[self.len()?, other.len()?])
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

    fn det(&self) -> Result<Array<N>, ArrayError> {
        if self.ndim()? == 0 {
            Err(ArrayError::MustBeAtLeast { value1: "`dimension`".to_string(), value2: "1".to_string() })
        } else if self.ndim()? == 1 {
            Ok(self.clone())
        } else if self.ndim()? == 2 {
            let shape = self.get_shape()?;
            shape[0].is_at_least(&2)?;
            shape[1].is_at_least(&2)?;
            shape[0].is_equal(&shape[1])?;
            if shape[0] == 2 {
                Array::single(N::from(self[0].to_f64() * self[3].to_f64() - self[1].to_f64() * self[2].to_f64()))
            } else {
                let elems = (0..shape[0])
                    .map(|i| self[i * shape[0]].to_f64())
                    .collect::<Vec<f64>>();
                let dets = (0..shape[0])
                    .map(|i| Self::minor(self, i, 0).det())
                    .collect::<Vec<Result<Array<N>, _>>>()
                    .has_error()?.into_iter()
                    .map(Result::unwrap)
                    .map(|i| i[0].to_f64())
                    .collect::<Vec<f64>>();
                let result = elems.iter().zip(&dets)
                    .enumerate()
                    .map(|(i, (&e, &d))| e * f64::powi(-1., i as i32 + 2) * d)
                    .sum::<f64>();
                Array::single(N::from(result))
            }
        } else {
            let shape = self.get_shape()?;
            shape[self.ndim()? - 2].is_at_least(&2)?;
            shape[self.ndim()? - 1].is_at_least(&2)?;
            shape[self.ndim()? - 2].is_equal(&shape[self.ndim()? - 1])?;
            let sub_shape = shape[self.ndim()? - 2 ..].to_vec();
            let dets = self
                .ravel()?
                .split(self.len()? / sub_shape.iter().product::<usize>(), None)?
                .iter()
                .map(|arr| {
                    println!("{arr}");
                    arr.reshape(&sub_shape).det()
                })
                .collect::<Vec<Result<Array<N>, _>>>()
                .has_error()?.into_iter()
                .flat_map(Result::unwrap)
                .collect::<Array<N>>();
            println!("{dets}");
            Ok(dets)
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

    fn inner(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.inner(other)
    }

    fn outer(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.outer(other)
    }

    fn matmul(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.matmul(other)
    }

    fn det(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.det()
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

    fn inner_nd<N: Numeric>(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        fn inner_split<N: Numeric>(arr: &Array<N>) -> Result<Vec<Array<N>>, ArrayError> {
            let _arr = arr.ravel()?;
            _arr.split(arr.get_shape()?.remove_at(arr.ndim()? - 1).iter().product(), None)
        }

        let mut new_shape = vec![];
        new_shape.extend_from_slice(&arr_1.get_shape()?.remove_at(arr_1.ndim()? - 1));
        new_shape.extend_from_slice(&arr_2.get_shape()?.remove_at(arr_2.ndim()? - 1));

        let _arr_1 = inner_split(arr_1)?;
        let _arr_2 = inner_split(arr_2)?;

        _arr_1.iter()
            .flat_map(|_a1| _arr_2.iter()
                .map(|_a2| _a1.inner(_a2))
                .collect::<Vec<Result<Array<N>, ArrayError>>>())
            .collect::<Vec<Result<Array<N>, ArrayError>>>()
            .has_error()?.into_iter()
            .flat_map(Result::unwrap)
            .collect::<Array<N>>()
            .reshape(&new_shape)
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

    fn minor<N: Numeric>(arr: &Array<N>, row: usize, col: usize) -> Result<Array<N>, ArrayError> {
        arr.is_dim_supported(&[2])?;
        if row >= arr.get_shape()?[0] || col >= arr.get_shape()?[1] {
            return Err(ArrayError::OutOfBounds { value: "Row or column index out of bounds" })
        }

        let mut sub_shape = arr.get_shape()?;
        sub_shape[arr.ndim()? - 1] -= 1;
        sub_shape[arr.ndim()? - 2] -= 1;

        let mut sub_elements = Vec::new();
        for (i, &element) in arr.get_elements()?.iter().enumerate() {
            if i / arr.get_shape()?[1] != row && i % arr.get_shape()?[1] != col {
                sub_elements.push(element);
            }
        }

        Array::new(sub_elements, sub_shape)
    }
}

impl <N: Numeric> LinalgHelper for Array<N> {}
