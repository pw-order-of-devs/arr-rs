use crate::traits::errors::ArrayError;

pub(crate) trait ValidateHasError {

    fn has_error(&self) -> Result<(), ArrayError>;
}

impl <T: Clone> ValidateHasError for Vec<Result<T, ArrayError>> {

    fn has_error(&self) -> Result<(), ArrayError> {
        let has_error = self.iter().find(|a| a.is_err());
        if let Some(error) = has_error { Err(error.clone().err().unwrap()) }
        else { Ok(()) }
    }
}
