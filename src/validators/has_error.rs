use crate::errors::ArrayError;

pub(crate) trait ValidateHasError where Self: Sized {

    fn has_error(&self) -> Result<Self, ArrayError>;
}

impl <T: Clone> ValidateHasError for Vec<Result<T, ArrayError>> {

    fn has_error(&self) -> Result<Self, ArrayError> {
        self.iter()
            .find(|a| a.is_err())
            .map_or_else(
            || Ok(self.clone()),
            |error| Err(error.clone().err().unwrap()))
    }
}
