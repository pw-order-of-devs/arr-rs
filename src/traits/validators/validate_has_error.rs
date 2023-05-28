use crate::traits::errors::ArrayError;

pub(crate) trait ValidateHasError where Self: Sized {

    fn has_error(&self) -> Result<Self, ArrayError>;
}

impl <T: Clone> ValidateHasError for Vec<Result<T, ArrayError>> {

    fn has_error(&self) -> Result<Self, ArrayError> {
        let has_error = self.iter().find(|a| a.is_err());
        if let Some(error) = has_error { Err(error.clone().err().unwrap()) }
        else { Ok(self.clone()) }
    }
}
