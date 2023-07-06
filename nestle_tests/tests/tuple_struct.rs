use nestle::core::Nestle;

use crate::common::{BirdNames, Birds, MyBirdsTuple};

mod common;

#[test]
fn test_named_struct() {
    assert_eq!(
        MyBirdsTuple(BirdNames::Donald, Birds::Eagle,)
            .encode()
            .unwrap(),
        0b0000000100000001000000000000000000000000000000000000000000000000_u64 as i64
    );
    assert_eq!(
        MyBirdsTuple::decode(
            0b0000000100000001000000000000000000000000000000000000000000000000_u64 as i64
        )
        .unwrap(),
        MyBirdsTuple(BirdNames::Donald, Birds::Eagle),
    );
    assert_eq!(
        MyBirdsTuple(BirdNames::Tweety, Birds::Pigeon,)
            .encode()
            .unwrap(),
        0b1111111111111111000000000000000000000000000000000000000000000000_u64 as i64
    );
    assert_eq!(
        MyBirdsTuple::decode(
            0b1111111111111111000000000000000000000000000000000000000000000000_u64 as i64
        )
        .unwrap(),
        MyBirdsTuple(BirdNames::Tweety, Birds::Pigeon),
    );
    assert!(MyBirdsTuple::decode(
        0b1111111111111111000000000000000000000000000000000000000000000001_u64 as i64
    )
    .is_err());
}
