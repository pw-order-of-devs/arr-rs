use crate::core::types::ArrayElement;

/// Array structure definition
#[derive(Clone, Debug)]
pub struct Array<T: ArrayElement> {
    pub(crate) elements: Vec<T>,
    pub(crate) shape: Vec<usize>,
}
