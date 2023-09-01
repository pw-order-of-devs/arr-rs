use crate::errors::prelude::*;

/// the kind of compare operation
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum CompareOp {
    /// equals
    Equals,
    /// not equals
    NotEquals,
    /// greater
    Greater,
    /// less
    Less,
    /// greater equal
    GreaterEqual,
    /// less equal
    LessEqual,
}

/// CompareOp trait
pub trait CompareOpType {

    /// Parse input to CompareOp type
    fn parse(self) -> Result<CompareOp, ArrayError>;
}

impl CompareOpType for CompareOp {

    fn parse(self) -> Result<CompareOp, ArrayError> {
        Ok(self)
    }
}

impl CompareOpType for &str {

    fn parse(self) -> Result<CompareOp, ArrayError> {
        parse_op(self.to_lowercase().as_str())
    }
}
impl CompareOpType for String {

    fn parse(self) -> Result<CompareOp, ArrayError> {
        parse_op(self.to_lowercase().as_str())
    }
}

fn parse_op(value: &str) -> Result<CompareOp, ArrayError> {
    match value.to_lowercase().as_str() {
        "==" | "equals" => Ok(CompareOp::Equals),
        "!=" | "not_equals" => Ok(CompareOp::NotEquals),
        ">" | "greater" => Ok(CompareOp::Greater),
        "<" | "less" => Ok(CompareOp::Less),
        ">=" | "greater_equal" => Ok(CompareOp::GreaterEqual),
        "<=" | "less_equal" => Ok(CompareOp::LessEqual),
        _ => Err(ArrayError::ParameterError { param: "`op`", message: "must be one of {`==`, `!=`, `>`, `<`, `>=`, `<=`}" })
    }
}
