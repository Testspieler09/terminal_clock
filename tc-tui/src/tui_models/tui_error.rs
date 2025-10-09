use std::fmt;

pub(crate) type UpdateResult<T> = Result<T, UpdateError>;

pub(crate) struct UpdateError;

impl fmt::Debug for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to update the selectable item")
    }
}
