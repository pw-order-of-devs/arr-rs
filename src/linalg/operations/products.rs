use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    linalg::prelude::*,
    math::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// `ArrayTrait` - Array Linalg Products functions
pub trait ArrayLinalgProducts<N: NumericOps> where Self: Sized + Clone {

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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn matmul(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Compute tensor dot product along specified axes
    ///
    /// # Arguments
    ///
    /// * `other` - other array to perform operations with
    /// * `axes` - int or (2,) array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(Array::single(70), Array::new(vec![1, 2, 3, 4], vec![2, 2]).tensordot(&Array::new(vec![5, 6, 7, 8], vec![2, 2]).unwrap(), Some(TensorAxes::Int(2))));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn tensordot(&self, other: &Array<N>, axes: Option<impl TensorAxesType>) -> Result<Array<N>, ArrayError>;
}

impl <N: NumericOps> ArrayLinalgProducts<N> for Array<N> {

    fn dot(&self, other: &Self) -> Result<Self, ArrayError> {
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

    fn vdot(&self, other: &Self) -> Result<Self, ArrayError> {
        self.len()?.is_equal(&other.len()?)?;
        let result = self.ravel()?.zip(&other.ravel()?)?
            .map(|tuple| tuple.0.to_f64() * tuple.1.to_f64())?
            .fold(0., |a, b| a + b)?;
        Self::single(N::from(result))
    }

    fn inner(&self, other: &Self) -> Result<Self, ArrayError> {
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

    fn outer(&self, other: &Self) -> Result<Self, ArrayError> {
        self.into_iter().flat_map(|a| other.into_iter()
            .map(|b| N::from(a.to_f64() * b.to_f64()))
            .collect::<Self>())
            .collect::<Self>()
            .reshape(&[self.len()?, other.len()?])
    }

    fn matmul(&self, other: &Self) -> Result<Self, ArrayError> {
        if self.ndim()? == 1 && other.ndim()? == 1 {
            self.vdot(other)
        } else if self.ndim()? == 1 || other.ndim()? == 1 {
            if self.ndim()? == 1 { self.shapes_align(0, &other.get_shape()?, other.ndim()? - 1)?; }
            else { self.shapes_align(self.ndim()? - 1, &other.get_shape()?, 0)?; }
            Self::matmul_1d_nd(self, other)
        } else if self.ndim()? == 2 && other.ndim()? == 2 {
            self.shapes_align(1, &other.get_shape()?, 0)?;
            Self::matmul_iterate(self, other)
        } else {
            Self::matmul_nd(self, other)
        }
    }

    fn tensordot(&self, other: &Self, axes: Option<impl TensorAxesType>) -> Result<Self, ArrayError> {

        fn normalize_axes<N: NumericOps>(array: &Array<N>, axes: &[isize]) -> Vec<usize> {
            axes.iter()
                .map(|&i| array.normalize_axis(i))
                .collect()
        }

        fn internal(axes: &[usize], shape: &[usize], ndim: usize, rev: bool) -> (Vec<isize>, Vec<usize>, Vec<usize>) {
            let notin = (0..ndim)
                .filter(|k| !axes.contains(k))
                .collect::<Vec<usize>>();
            let newaxes: Vec<usize> =
                if rev { axes.iter().chain(notin.iter()).copied().collect() }
                else { notin.iter().chain(axes.iter()).copied().collect() };
            let newaxes = newaxes.iter()
                .map(|&i| i.to_isize())
                .collect();

            let mut n2 = 1;
            for &axis in axes { n2 *= shape[axis] }
            let newshape = notin.iter()
                .map(|&ax| shape[ax])
                .collect::<Vec<usize>>()
                .iter()
                .copied().product::<usize>();
            let newshape =
                if rev { vec![n2, newshape] }
                else { vec![newshape, n2] };
            let old = notin.iter()
                .map(|&ax| shape[ax.to_usize()])
                .collect::<Vec<usize>>();

            (newaxes, newshape, old)
        }

        let (a_axes, b_axes) = match axes {
            Some(ta) => ta.to_axes()?,
            None => TensorAxes::Int(2),
        }.get_vecs()?;
        let (a_axes, b_axes) = (
            normalize_axes(self, &a_axes),
            normalize_axes(other, &b_axes));

        let (na, nb) = (a_axes.len(), b_axes.len());
        let (a_shape, a_ndim) = (self.get_shape()?, self.ndim()?);
        let (b_shape, b_ndim) = (other.get_shape()?, other.ndim()?);
        if na != nb { return Err(ArrayError::ShapesMustMatch { shape_1: a_shape, shape_2: b_shape, }); }
        for k in 0..na {
            if a_shape[a_axes[k]] != b_shape[b_axes[k]] {
                return Err(ArrayError::ShapesMustMatch { shape_1: a_shape, shape_2: b_shape, })
            }
        }

        let (newaxes_a, newshape_a, olda) = internal(&a_axes, &a_shape, a_ndim, false);
        let (newaxes_b, newshape_b, oldb) = internal(&b_axes, &b_shape, b_ndim, true);
        let a_transposed = self.transpose(Some(newaxes_a)).reshape(&newshape_a)?;
        let b_transposed = other.transpose(Some(newaxes_b)).reshape(&newshape_b)?;

        let mut new_shape = olda;
        new_shape.extend_from_slice(&oldb);
        if new_shape.is_empty() { new_shape = vec![1] }
        a_transposed.dot(&b_transposed).reshape(&new_shape)
    }
}

impl <N: NumericOps> ArrayLinalgProducts<N> for Result<Array<N>, ArrayError> {

    fn dot(&self, other: &Array<N>) -> Self {
        self.clone()?.dot(other)
    }

    fn vdot(&self, other: &Array<N>) -> Self {
        self.clone()?.vdot(other)
    }

    fn inner(&self, other: &Array<N>) -> Self {
        self.clone()?.inner(other)
    }

    fn outer(&self, other: &Array<N>) -> Self {
        self.clone()?.outer(other)
    }

    fn matmul(&self, other: &Array<N>) -> Self {
        self.clone()?.matmul(other)
    }

    fn tensordot(&self, other: &Array<N>, axes: Option<impl TensorAxesType>) -> Self {
        self.clone()?.tensordot(other, axes)
    }
}

trait ProductsHelper<N: NumericOps> {

    fn dot_split_array(arr: &Array<N>, axis: usize) -> Result<Vec<Array<N>>, ArrayError> {
        arr.split_axis(axis)?
            .into_iter().flatten()
            .collect::<Array<N>>()
            .split(arr.get_shape()?.remove_at(axis).iter().product(), None)
    }

    fn dot_iterate(v_arr_1: &[Array<N>], v_arr_2: &[Array<N>]) -> Result<Array<N>, ArrayError> {
        v_arr_1.iter().flat_map(|a| {
            v_arr_2.iter().map(move |b| a.vdot(b))
        })
            .collect::<Vec<Result<Array<N>, _>>>()
            .has_error()?.into_iter()
            .flat_map(Result::unwrap)
            .collect::<Array<N>>()
            .ravel()
    }

    fn dot_1d(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        let arr_1 = if arr_1.ndim()? > 1 { arr_1.get_rows()? } else { vec![arr_1.clone()] };
        let arr_2 = if arr_2.ndim()? > 1 { arr_2.get_columns()? } else { vec![arr_2.clone()] };
        Self::dot_iterate(&arr_1, &arr_2)
    }

    fn dot_nd(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        arr_1.shapes_align(arr_1.ndim()? - 1, &arr_2.get_shape()?, arr_2.ndim()? - 2)?;
        let mut new_shape = arr_1.get_shape()?.remove_at(arr_1.ndim()? - 2);
        new_shape.extend_from_slice(&arr_2.get_shape()?.remove_at(arr_2.ndim()? - 1));
        let v_arr_1 = Self::dot_split_array(arr_1, arr_1.ndim()? - 2)?;
        let v_arr_2 = Self::dot_split_array(arr_2, arr_2.ndim()? - 1)?;

        let rev = arr_2.len()? > arr_1.len()?;
        let pairs = (0..new_shape.len().to_isize())
            .collect::<Vec<isize>>()
            .reverse_if(rev)
            .into_iter()
            .step_by(2)
            .map(|item|
                if rev { if item <= 1 { vec![item] } else { vec![item, item - 1] } }
                else if new_shape.len().to_isize() > item + 1 { vec![item + 1, item] }
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

    fn inner_nd(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        fn inner_split<N: NumericOps>(arr: &Array<N>) -> Result<Vec<Array<N>>, ArrayError> {
            let r_arr = arr.ravel()?;
            r_arr.split(arr.get_shape()?.remove_at(arr.ndim()? - 1).iter().product(), None)
        }

        let mut new_shape = vec![];
        new_shape.extend_from_slice(&arr_1.get_shape()?.remove_at(arr_1.ndim()? - 1));
        new_shape.extend_from_slice(&arr_2.get_shape()?.remove_at(arr_2.ndim()? - 1));

        let v_arr_1 = inner_split(arr_1)?;
        let v_arr_2 = inner_split(arr_2)?;

        v_arr_1.iter()
            .flat_map(|v_a1| v_arr_2.iter()
                .map(|v_a2| v_a1.inner(v_a2))
                .collect::<Vec<Result<Array<N>, ArrayError>>>())
            .collect::<Vec<Result<Array<N>, ArrayError>>>()
            .has_error()?.into_iter()
            .flat_map(Result::unwrap)
            .collect::<Array<N>>()
            .reshape(&new_shape)
    }

    fn matmul_iterate(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        let (shape_1, shape_2) = (&arr_1.get_shape()?, &arr_2.get_shape()?);
        (0..shape_1[0])
            .flat_map(|i| (0..shape_2[1])
                .map(move |j| (0..shape_1[1])
                    .fold(0., |acc, k| arr_1[i * shape_1[1] + k].to_f64().mul_add(arr_2[k * shape_2[1] + j].to_f64(), acc))))
            .map(N::from_f64)
            .collect::<Array<N>>()
            .reshape(&[shape_1[0], shape_2[1]])
    }

    fn matmul_1d_nd(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
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

    fn matmul_nd(arr_1: &Array<N>, arr_2: &Array<N>) -> Result<Array<N>, ArrayError> {
        fn matmul_split<N: NumericOps>(arr: &Array<N>, len: usize, chunk_len: usize) -> Result<Vec<Array<N>>, ArrayError> {
            let shape_last = arr.get_shape()?
                .into_iter()
                .skip(arr.ndim()? - 2)
                .take(2)
                .collect::<Vec<usize>>();
            let result = arr.split(arr.len()? / chunk_len, Some(0))?
                .into_iter().cycle().take(len)
                .map(|arr| arr.reshape(&shape_last).unwrap())
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

impl <N: NumericOps> ProductsHelper<N> for Array<N> {}
