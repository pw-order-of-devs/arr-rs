/// prelude module - imports facade
pub mod prelude;

/// `ArrayError` definition - errors that can be returned by the library
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
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
    /// Values must be at least
    MustBeAtLeast {
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
    /// Matrix is singular
    SingularMatrix,
}

impl std::error::Error for ArrayError {}

impl std::fmt::Display for ArrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BroadcastShapeMismatch => write!(f, "Incompatible shapes for broadcasting"),
            Self::ConcatenateShapeMismatch => write!(f, "Incompatible shapes for concatenate"),
            Self::ShapeMustMatchValuesLength => write!(f, "Shape must match values length"),
            Self::ShapesMustMatch { shape_1, shape_2 } => write!(f, "Shapes of {shape_1:?} and {shape_2:?} must match"),
            Self::SqueezeShapeOfAxisMustBeOne => write!(f, "cannot select an axis to squeeze out which has size not equal to one"),
            Self::AxisOutOfBounds => write!(f, "`axis` is out of bounds for array"),
            Self::OutOfBounds { value } => write!(f, "`{value}` is out of bounds"),
            Self::ParameterError { param, message } => write!(f, "parameter error: `{param}`: {message}"),
            Self::UnsupportedDimension { supported } => write!(f, "supported dimensions are: {supported:?}"),
            Self::MustBeUnique { value } => write!(f, "`{value}` must be unique"),
            Self::MustBeEqual { value1, value2 } => write!(f, "`{value1}` and `{value2}` must be equal"),
            Self::MustBeAtLeast { value1, value2 } => write!(f, "`{value1}` must be at least `{value2}`"),
            Self::MustBeOneOf { value1, value2 } => write!(f, "`{value1}` must be one of `{value2}`"),
            Self::NotImplemented => write!(f, "not implemented"),
            Self::SingularMatrix => write!(f, "matrix is singular"),
        }
    }
}
