use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    linalg::prelude::*,
    math::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// `ArrayTrait` - Array Linalg Norms functions
pub trait ArrayLinalgNorms<N: NumericOps> where Self: Sized + Clone {

    /// Calculates matrix or vector norm
    ///
    /// # Arguments
    ///
    /// * `ord` - order of the norm: {non-zero int, inf, -inf, `fro`, `nuc`}. optional
    /// * `axis` - axis along which vector norms are to be calculated. optional
    /// * `keepdims` - if true, the result will broadcast correctly against the input. optional
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let array_a = Array::arange(-4., 4., None);
    /// let array_b = array_a.reshape(&[3, 3]);
    ///
    /// let expected = Array::single(7.745966692414834);
    /// assert_eq!(expected, array_a.norm(None::<NormOrd>, None, None));
    /// assert_eq!(expected, array_b.norm(None::<NormOrd>, None, None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn norm(&self, ord: Option<impl NormOrdType>, axis: Option<Vec<isize>>, keepdims: Option<bool>) -> Result<Array<N>, ArrayError>;

    /// Compute the determinant of an array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(Array::single(-14), Array::new(vec![3, 8, 4, 6], vec![2, 2]).det());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn det(&self) -> Result<Array<N>, ArrayError>;

    /// Return matrix rank of array using SVD method
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(Array::single(4), Array::<f64>::eye(4, None, None).matrix_rank());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn matrix_rank(&self) -> Result<Array<usize>, ArrayError>;
}

impl <N: NumericOps> ArrayLinalgNorms<N> for Array<N> {

    fn norm(&self, ord: Option<impl NormOrdType>, axis: Option<Vec<isize>>, keepdims: Option<bool>) -> Result<Self, ArrayError> {

        fn norm_simple<N: NumericOps>(array: &Array<N>, keepdims: Option<bool>) -> Result<Array<N>, ArrayError> {
            let ndim = array.ndim()?;
            let array = array.ravel()?;
            let result = array
                .dot(&array)
                .sqrt();
            if keepdims.unwrap_or(false) { result.reshape(&[ndim; 1]) }
            else { result }
        }

        let ndim = self.ndim()?;
        if axis.is_none() {
            match ord.clone() {
                Some(ord) => {
                    let ord = ord.to_ord()?;
                    if (ndim == 2 && ord == NormOrd::Fro) || (ndim == 1 && ord == NormOrd::Int(2)) {
                        return norm_simple(self, keepdims)
                    }
                },
                None => return norm_simple(self, keepdims)
            }
        }

        let axis = axis.unwrap_or_else(|| (0..ndim.to_isize()).collect());
        match axis.len() {
            1 => {
                let axis = Some(axis[0]);
                let ord = match ord {
                    Some(o) => o.to_ord()?,
                    None => NormOrd::Int(2),
                };
                match ord {
                    NormOrd::Inf => self.abs().max(axis),
                    NormOrd::NegInf => self.abs().min(axis),
                    NormOrd::Int(0) => self.map(|&i| if i == N::zero() { N::zero() } else { N::one() }).sum(axis),
                    NormOrd::Int(1) => self.abs().sum(axis),
                    NormOrd::Int(2) => self.multiply(self).abs()?.sum(axis).sqrt(),
                    NormOrd::Int(value) => self.abs()
                        .float_power(&Self::single(N::from(value))?)
                        .sum(axis)
                        .float_power(&Self::single(N::from(value)).reciprocal()?),
                    NormOrd::Fro | NormOrd::Nuc => {
                        Err(ArrayError::ParameterError { param: "`ord`", message: "invalid norm order for vectors." })
                    },
                }
            }
            2 => {
                let row_axis = self.normalize_axis(axis[0]).to_isize();
                let col_axis = self.normalize_axis(axis[1]).to_isize();
                if row_axis == col_axis {
                    return Err(ArrayError::ParameterError { param: "`axis`", message: "duplicate axes given." });
                }
                let ord = match ord {
                    Some(o) => o.to_ord()?,
                    None => NormOrd::Fro,
                };
                let result = match ord {
                    NormOrd::Int(1) => {
                        let col_axis = if col_axis > row_axis { -col_axis } else { col_axis };
                        self.abs().sum(Some(row_axis)).max(Some(col_axis))
                    },
                    NormOrd::Int(-1) => {
                        let col_axis = if col_axis > row_axis { -col_axis } else { col_axis };
                        self.abs().sum(Some(row_axis)).min(Some(col_axis))
                    },
                    NormOrd::Inf => {
                        let row_axis = if row_axis > col_axis { -row_axis } else { row_axis };
                        self.abs().sum(Some(col_axis)).max(Some(row_axis))
                    },
                    NormOrd::NegInf => {
                        let row_axis = if row_axis > col_axis { -row_axis } else { row_axis };
                        self.abs().sum(Some(col_axis)).min(Some(row_axis))
                    },
                    _ => {
                        Err(ArrayError::ParameterError { param: "`ord`", message: "invalid norm order for vectors." })
                    },
                };
                if keepdims.unwrap_or(false) {
                    let mut new_shape = result.get_shape()?;
                    new_shape.push(1);
                    result.reshape(&new_shape)
                } else {
                    result
                }
            }
            _ => Err(ArrayError::ParameterError { param: "`axis`", message: "improper number of dimensions to norm." })
        }
    }

    fn det(&self) -> Result<Self<>, ArrayError> {
        if self.ndim()? == 0 {
            Err(ArrayError::MustBeAtLeast { value1: "`dimension`".to_string(), value2: "1".to_string() })
        } else if self.ndim()? == 1 {
            Ok(self.clone())
        } else if self.ndim()? == 2 {
            let shape = self.get_shape()?;
            self.is_square()?;
            if shape[0] == 2 {
                Self::single(N::from(self[0].to_f64().mul_add(self[3].to_f64(), -self[1].to_f64() * self[2].to_f64())))
            } else {
                let elems = (0..shape[0])
                    .map(|i| self[i * shape[0]].to_f64())
                    .collect::<Vec<f64>>();
                let dets = (0..shape[0])
                    .map(|i| Self::minor(self, i, 0).det())
                    .collect::<Vec<Result<Self, _>>>()
                    .has_error()?.into_iter()
                    .map(Result::unwrap)
                    .map(|i| i[0].to_f64())
                    .collect::<Vec<f64>>();
                let result = elems.iter().zip(&dets)
                    .enumerate()
                    .map(|(i, (&e, &d))| e * f64::powi(-1., i.to_i32() + 2) * d)
                    .sum::<f64>();
                Self::single(N::from(result))
            }
        } else {
            let shape = self.get_shape()?;
            shape.is_square()?;
            let sub_shape = shape[self.ndim()? - 2 ..].to_vec();
            let dets = self
                .ravel()?
                .split(self.len()? / sub_shape.iter().product::<usize>(), None)?
                .iter()
                .map(|arr| arr.reshape(&sub_shape).det())
                .collect::<Vec<Result<Self, _>>>()
                .has_error()?.into_iter()
                .flat_map(Result::unwrap)
                .collect::<Self>();
            Ok(dets)
        }
    }

    fn matrix_rank(&self) -> Result<Array<usize>, ArrayError> {
        if self.ndim()? < 2 {
            Array::single(self.iter().all(|&i| i != N::zero()).to_usize())
        } else {
            self.to_array_f64()
                .svd()?.s
                .count_nonzero(None, None)
        }
    }
}

impl <N: NumericOps> ArrayLinalgNorms<N> for Result<Array<N>, ArrayError> {

    fn norm(&self, ord: Option<impl NormOrdType>, axis: Option<Vec<isize>>, keepdims: Option<bool>) -> Self {
        self.clone()?.norm(ord, axis, keepdims)
    }

    fn det(&self) -> Self {
        self.clone()?.det()
    }

    fn matrix_rank(&self) -> Result<Array<usize>, ArrayError> {
        self.clone()?.matrix_rank()
    }
}

trait NormsHelper<N: NumericOps> {

    fn minor(arr: &Array<N>, row: usize, col: usize) -> Result<Array<N>, ArrayError> {
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

impl <N: NumericOps> NormsHelper<N> for Array<N> {}
