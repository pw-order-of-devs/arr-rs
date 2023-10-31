use crate::prelude::*;

pub(crate) trait ArrayCreateExt<T: ArrayElement> {

    fn to_array(&self) -> Result<Array<T>, ArrayError>;
}

impl <T: ArrayElement> ArrayCreateExt<T> for T {

    fn to_array(&self) -> Result<Array<T>, ArrayError> {
        Array::single(self.clone())
    }
}

impl <T: ArrayElement> ArrayCreateExt<T> for Vec<T> {

    fn to_array(&self) -> Result<Array<T>, ArrayError> {
        Array::flat(self.clone())
    }
}

pub(crate) trait ArrayNumericExt<N: Numeric> {

    fn to_array_num<S: Numeric>(&self) -> Result<Array<S>, ArrayError>;
    fn to_array_f64(&self) -> Result<Array<f64>, ArrayError>;
}

impl <N: Numeric> ArrayNumericExt<N> for Array<N> {

    fn to_array_num<S: Numeric>(&self) -> Result<Array<S>, ArrayError> {
        self.get_elements()?.into_iter()
            .map(|i| S::from(i.to_f64()))
            .collect::<Array<S>>()
            .reshape(&self.get_shape()?)
    }

    fn to_array_f64(&self) -> Result<Array<f64>, ArrayError> {
        self.get_elements()?.into_iter()
            .map(|i| i.to_f64())
            .collect::<Array<f64>>()
            .reshape(&self.get_shape()?)
    }
}

pub(crate) trait ArrayExt<T: ArrayElement> {

    fn to_array_ndim(&self, ndim: usize) -> Result<Array<T>, ArrayError>;
}

impl <T: ArrayElement> ArrayExt<T> for Array<T> {

    fn to_array_ndim(&self, ndim: usize) -> Result<Self, ArrayError> {
        Self::create(self.get_elements()?, self.get_shape()?, Some(ndim))
    }
}
