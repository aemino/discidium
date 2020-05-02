use thiserror::Error;

/// "Polyfill" for nightly's `NoneError`, minus the `Try` impl (which is also unstable).
#[derive(Debug, Error)]
#[error("NoneError")]
pub struct NoneError;
