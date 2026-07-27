use std::fmt;

#[allow(dead_code)]
pub(crate) type UpdateResult<T> = Result<T, UpdateError>;

#[allow(dead_code)]
pub(crate) struct UpdateError;

impl fmt::Debug for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to update the selectable item")
    }
}
