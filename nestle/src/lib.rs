//! # Nestle
//!
//! Encode/decode arbitrary nested tree-structured data with a single
//! integer.
//!
//! ## Usage
//!
//! Simply use the [Nestle](derives::Nestle) derive macro on your enum or
//! struct.
//!
//! If the width of the top-level type is exceeded, an error will be
//! returned at the point of encoding.
//!
//! ```
//! # use nestle::core::Nestle;
//! # use nestle::derives::Nestle;
//! // This is the top-level type, so it should be 64 bits wide. It uses
//! // `repr(i16)` therefore occupies 16 bits, leaving 48 bits for the
//! // descendants.
//! #[derive(Nestle)]
//! # #[derive(Debug, PartialEq)]
//! #[nestle(width = 64)]
//! #[repr(i16)]
//! enum Instrument {
//!     Spot(CurrencyPair) = 1,
//!     Options(Options) = 2,
//!     PerpetualFutures(CurrencyPair) = 3,
//! }
//!
//! // This type is 48 bits wide, so it can be encoded as a descendant of
//! // `Instrument`.
//! #[derive(Nestle)]
//! # #[derive(Debug, PartialEq)]
//! #[nestle(width = 48)]
//! struct Options {
//!     expiry: u16,
//!     pair: CurrencyPair,
//! }
//!
//! // This type doesn't use all the available space, which is fine.
//! #[derive(Nestle)]
//! # #[derive(Debug, PartialEq)]
//! #[nestle(width = 32)]
//! struct CurrencyPair {
//!     base: Currency,
//!     quote: Currency,
//! }
//!
//! // The discriminants of this enum will be used to encode the values.
//! #[derive(Nestle)]
//! # #[derive(Debug, PartialEq)]
//! #[repr(i16)]
//! enum Currency {
//!     Btc = 1,
//!     Eth = 2,
//!     Doge = 3,
//!     Usd = 4,
//! }
//!
//! # fn main() {
//! let instrument = Instrument::Options(Options {
//!     expiry: 365,
//!     pair: CurrencyPair {
//!         base: Currency::Eth,
//!         quote: Currency::Usd,
//!     },
//! });
//!
//! let instrument_id = 564517616615428;
//! // This is the unique, semantic identifier for this instrument.
//! assert_eq!(instrument.encode().unwrap(), instrument_id);
//! // It's the result of the discriminants shifted in to the appropriate
//! // bit positions.
//! assert_eq!(
//!     instrument.encode().unwrap(),
//!     2 << 48 | 365 << 32 | 2 << 16 | 4
//! );
//!
//! // The decode method will return the original value.
//! assert_eq!(Instrument::decode(instrument_id).unwrap(), instrument);
//! // Ids have semantic meaning, giving them reproducibility and
//! // mathematical properties.
//! assert_eq!(
//!     Instrument::decode(instrument_id - 1).unwrap(),
//!     Instrument::Options(Options {
//!         expiry: 365,
//!         pair: CurrencyPair {
//!             base: Currency::Eth,
//!             quote: Currency::Doge,
//!         },
//!     })
//! );
//! // The decode method will fail only if the id is not a valid encoding.
//! assert!(Instrument::decode(instrument_id + 1).is_err());
//! # }
//! ```
#![cfg_attr(doc_cfg, feature(doc_cfg, doc_auto_cfg))]

pub mod core {
    pub use nestle_core::*;
}
pub mod derives {
    pub use nestle_derives::*;
}
