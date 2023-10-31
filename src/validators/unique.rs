use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::errors::ArrayError;

pub(crate) trait ValidateUnique {

    fn is_unique(&self) -> Result<(), ArrayError>;
}

impl <T: Clone + Debug + Eq + Hash> ValidateUnique for Vec<T> {

    fn is_unique(&self) -> Result<(), ArrayError> {
        let unique_len = self.iter().cloned().collect::<HashSet<T>>().len();
        if self.len() == unique_len {
            Ok(())
        } else {
            Err(ArrayError::MustBeUnique { value: format!("{:?}", self.clone()) })
        }
    }
}
