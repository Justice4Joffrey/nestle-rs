use thiserror::Error;

/// This value is too wide to fit into the remaining bits.
#[derive(Debug, Error)]
#[error("{type_name} requires {width} bits but only {remaining} bits remain at offset {offset}")]
pub struct ValueTooWide {
    pub type_name: &'static str,
    pub offset: u8,
    pub width: u8,
    pub remaining: u8,
}

/// It's impossible for encoding to fail if the `Nestle` trait has been
/// implemented exclusively through the derive macros.
#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("{0}")]
    ValueTooWide(#[from] ValueTooWide),
}

/// Failure modes decoding an integer id.
#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("{0}")]
    ValueTooWide(#[from] ValueTooWide),
    #[error("Type {typ} has no value {disc}")]
    NotFound { typ: &'static str, disc: i64 },
    #[error("Successfully parsed {typ} but trailing bits were not zero: {value}")]
    TrailingBits { typ: &'static str, value: i64 },
}
