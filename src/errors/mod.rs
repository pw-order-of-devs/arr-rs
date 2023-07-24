/// prelude module - imports facade
pub mod prelude;

/// ArrayError definition - errors that can be returned by the library
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ArrayError {
    /// Shape of two arrays must match for broadcast operation
    BroadcastShapeMismatch,
    /// Shape of two arrays must match for concatenate operation
    ConcatenateShapeMismatch,
    /// Shape mismatching the length of values to build the array from
    ShapeMustMatchValuesLength,
    /// Shape of two arrays must match
    ShapesMustMatch {
        /// first shape to match
        shape_1: Vec<usize>,
        /// second shape to match
        shape_2: Vec<usize>,
    },
    /// Axis to perform squeeze on must have a size of one
    SqueezeShapeOfAxisMustBeOne,
    /// `axis` is out of array's bounds
    AxisOutOfBounds,
    /// `value` is out of bounds
    OutOfBounds {
        /// out of bounds value
        value: &'static str,
    },
    /// Input parameter doesn't match the criteria
    ParameterError {
        /// mismatching parameter
        param: &'static str,
        /// parameter error message
        message: &'static str,
    },
    /// Not defined for given dimension of input array
    UnsupportedDimension {
        /// supported dimensions
        supported: Vec<usize>,
    },
    /// Collection must contain unique elements
    MustBeUnique {
        /// value
        value: String,
    },
    /// Values must be equal
    MustBeEqual {
        /// value 1
        value1: String,
        /// value 2
        value2: String,
    },
    /// Values must be one of
    MustBeOneOf {
        /// value 1
        value1: String,
        /// value 2
        value2: String,
    },
    /// Function is not implemented
    NotImplemented,
}

impl std::error::Error for ArrayError {}

impl std::fmt::Display for ArrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArrayError::BroadcastShapeMismatch => write!(f, "Incompatible shapes for broadcasting"),
            ArrayError::ConcatenateShapeMismatch => write!(f, "Incompatible shapes for concatenate"),
            ArrayError::ShapeMustMatchValuesLength => write!(f, "Shape must match values length"),
            ArrayError::ShapesMustMatch { shape_1, shape_2 } => write!(f, "Shapes of {shape_1:?} and {shape_2:?} must match"),
            ArrayError::SqueezeShapeOfAxisMustBeOne => write!(f, "cannot select an axis to squeeze out which has size not equal to one"),
            ArrayError::AxisOutOfBounds => write!(f, "`axis` is out of bounds for array"),
            ArrayError::OutOfBounds { value } => write!(f, "`{value}` is out of bounds"),
            ArrayError::ParameterError { param, message } => write!(f, "parameter error: `{param}`: {message}"),
            ArrayError::UnsupportedDimension { supported } => write!(f, "supported dimensions are: {supported:?}"),
            ArrayError::MustBeUnique { value } => write!(f, "`{value}` must be unique"),
            ArrayError::MustBeEqual { value1, value2 } => write!(f, "`{value1}` and `{value2}` must be equal"),
            ArrayError::MustBeOneOf { value1, value2 } => write!(f, "`{value1}` must be one of `{value2}`"),
            ArrayError::NotImplemented => write!(f, "not implemented"),
        }
    }
}
