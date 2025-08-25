//! ALL Errors in the project
#[derive(Debug, thiserror::Error)]
#[error("An Application error has occurred")]
pub struct AppError;

// A suggestion error display to the user
pub struct SuggestionError(pub &'static str);
