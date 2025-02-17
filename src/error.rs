/// Main error type
///
/// It doesn't provide any useful information yet.
///
/// It will probably be improved in the futur to make possible
/// user-friendly error messages.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Error;

use crate::ConversionError;

impl From<ConversionError> for Error {
    fn from(_: ConversionError) -> Self {
        Self
    }
}
