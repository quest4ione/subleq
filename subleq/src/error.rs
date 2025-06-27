use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("address `{0}` is out of range for memory")]
    AddressOutOfRange(usize),
    #[error("immutable memory address `{0}`")]
    ImmutableAddress(usize),
    #[error("custom error: {0}")]
    Custom(#[source] Box<dyn std::error::Error>),
}
