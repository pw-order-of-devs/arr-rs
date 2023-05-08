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
        s1: &'static str,
        /// second shape to match
        s2: &'static str,
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
        /// function name
        fun: &'static str,
        /// supported dimensions
        supported: &'static str,
    },
}

impl std::error::Error for ArrayError {}

impl std::fmt::Display for ArrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArrayError::BroadcastShapeMismatch => write!(f, "Incompatible shapes for broadcasting"),
            ArrayError::ConcatenateShapeMismatch => write!(f, "Incompatible shapes for concatenate"),
            ArrayError::ShapeMustMatchValuesLength => write!(f, "Shape must match values length"),
            ArrayError::ShapesMustMatch { s1, s2 } => write!(f, "Shapes of {s1} and {s2} must match"),
            ArrayError::SqueezeShapeOfAxisMustBeOne => write!(f, "cannot select an axis to squeeze out which has size not equal to one"),
            ArrayError::AxisOutOfBounds => write!(f, "`axis` is out of bounds for array"),
            ArrayError::OutOfBounds { value } => write!(f, "`{value}` is out of bounds"),
            ArrayError::ParameterError { param, message } => write!(f, "parameter error: `{param}`: {message}"),
            ArrayError::UnsupportedDimension { fun, supported } => write!(f, "`{fun}` is defined only for {supported} dimensions"),
        }
    }
}
