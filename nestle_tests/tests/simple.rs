use nestle::core::Nestle;

use crate::common::Birds;

mod common;

#[test]
fn test_simple_enum() {
    assert_eq!(Birds::Eagle.encode_unchecked(), 1 << 56);
    assert_eq!(Birds::Albatross.encode_unchecked(), 2 << 56);
    assert_eq!(Birds::Hawk.encode_unchecked(), 3 << 56);
    assert_eq!(Birds::Pigeon.encode_unchecked(), -1 << 56);
    assert_eq!(Birds::Dove.encode_unchecked(), -2 << 56);
    assert_eq!(Birds::Swallow.encode_unchecked(), -3 << 56);

    assert!(Birds::decode(0).is_err());
    assert_eq!(Birds::decode((1_u64 << 56) as i64).unwrap(), Birds::Eagle);
    assert_eq!(
        Birds::decode((2_u64 << 56) as i64).unwrap(),
        Birds::Albatross
    );
    assert_eq!(Birds::decode((3_u64 << 56) as i64).unwrap(), Birds::Hawk);
    assert_eq!(Birds::decode((-1_i64 << 56) as i64).unwrap(), Birds::Pigeon);
    assert_eq!(Birds::decode((-2_i64 << 56) as i64).unwrap(), Birds::Dove);
    assert_eq!(
        Birds::decode((-3_i64 << 56) as i64).unwrap(),
        Birds::Swallow
    );
    assert!(Birds::decode(4).is_err());
}
