use crate::error::{DecodeError, EncodeError, ValueTooWide};

pub mod error;
pub mod macros;

pub trait Nestle: Sized {
    /// The total bit-width of this type including all fields.
    const WIDTH: u8;
    /// An identifier for this type. Useful for debugging decoding/encoding
    /// errors.
    const TYPE_NAME: &'static str;

    /// Decode this type from the given `id` from `offset` bytes.
    fn decode_from(id: i64, offset: &mut u8) -> Result<Self, DecodeError>;

    /// Encode this type into the given `id` from `offset` bytes.
    fn encode_into(&self, id: &mut i64, offset: u8) -> Result<(), EncodeError>;

    /// Decode this type as the top-level type.
    fn decode(id: i64) -> Result<Self, DecodeError> {
        let mut offset = 0;
        let res = Self::decode_from(id, &mut offset)?;
        // check any trailing bits are zero
        let mask = {
            let (res, overflow) = 1_i64.overflowing_shl(64 - offset as u32);
            if overflow {
                0
            } else {
                res
            }
        };
        let value = id & (mask - 1);
        if value == 0 {
            Ok(res)
        } else {
            Err(DecodeError::TrailingBits {
                typ: Self::TYPE_NAME,
                value,
            })
        }
    }

    /// Encode this type as the top-level type.
    fn encode(&self) -> Result<i64, EncodeError> {
        let mut id = 0;
        self.encode_into(&mut id, 0)?;
        Ok(id)
    }

    /// Any types using the macros to implement `Nestle` are guaranteed to be
    /// able to decode from an `i64` without error.
    fn encode_unchecked(&self) -> i64 {
        self.encode().unwrap()
    }
}

pub fn check_i64_width(type_name: &'static str, width: u8, offset: u8) -> Result<(), ValueTooWide> {
    let remaining = 64 - offset;
    if width <= remaining {
        Ok(())
    } else {
        Err(ValueTooWide {
            type_name,
            offset,
            width,
            remaining,
        })
    }
}

macro_rules! impl_for_primitive {
    ($typ:ty, $cst:ty, $width:literal) => {
        impl Nestle for $typ {
            const TYPE_NAME: &'static str = stringify!($typ);
            const WIDTH: u8 = $width;

            fn decode_from(id: i64, offset: &mut u8) -> Result<Self, DecodeError> {
                check_i64_width(Self::TYPE_NAME, Self::WIDTH, *offset)?;
                let shift = 64 - *offset - Self::WIDTH;
                let shifted = id >> shift;
                let res = {
                    let (res, overflow) = 1_i64.overflowing_shl(Self::WIDTH as u32);
                    if overflow {
                        0
                    } else {
                        res
                    }
                };
                let value = (shifted & (res - 1)) as $typ;
                *offset += Self::WIDTH;
                Ok(value)
            }

            fn encode_into(&self, id: &mut i64, offset: u8) -> Result<(), EncodeError> {
                check_i64_width(Self::TYPE_NAME, Self::WIDTH, offset)?;
                let shift = 64 - offset - Self::WIDTH;
                *id |= ((*self as $cst as u64 as i64) << shift) as i64;
                Ok(())
            }
        }
    };
}

impl_for_primitive!(i8, u8, 8);
impl_for_primitive!(i16, u16, 16);
impl_for_primitive!(i32, u32, 32);
impl_for_primitive!(i64, u64, 64);
