use nestle::core::Nestle;

use crate::common::{BirdNames, Birds, Mammals, MyAnimals, MyBirdsTuple, NumberTuple};

mod common;

#[test]
fn test_data_enum() {
    assert_eq!(
        MyAnimals::Birds2(MyBirdsTuple(BirdNames::Donald, Birds::Eagle,))
            .encode()
            .unwrap(),
        0b0000001000000001000000010000000000000000000000000000000000000000_u64 as i64
    );
    assert_eq!(
        MyAnimals::decode(
            0b0000001000000001000000010000000000000000000000000000000000000000_u64 as i64
        )
        .unwrap(),
        MyAnimals::Birds2(MyBirdsTuple(BirdNames::Donald, Birds::Eagle,)),
    );
    assert_eq!(
        MyAnimals::Mammals(Mammals::Horse).encode().unwrap(),
        0b1111111111111111000000000000000000000000000000000000000000000000_u64 as i64
    );
    assert_eq!(
        MyAnimals::decode(
            0b1111111111111111000000000000000000000000000000000000000000000000_u64 as i64
        )
        .unwrap(),
        MyAnimals::Mammals(Mammals::Horse),
    );
    assert_eq!(
        MyAnimals::Number(64).encode().unwrap(),
        0b1111110101000000000000000000000000000000000000000000000000000000_u64 as i64
    );
    assert_eq!(
        MyAnimals::decode(
            0b1111110101000000000000000000000000000000000000000000000000000000_u64 as i64
        )
        .unwrap(),
        MyAnimals::Number(64),
    );
    assert_eq!(
        MyAnimals::NumberTuple(NumberTuple(32, 8)).encode().unwrap(),
        0b1111110000100000000010000000000000000000000000000000000000000000_u64 as i64
    );
    assert_eq!(
        MyAnimals::decode(
            0b1111110000100000000010000000000000000000000000000000000000000000_u64 as i64
        )
        .unwrap(),
        MyAnimals::NumberTuple(NumberTuple(32, 8)),
    );
    assert_eq!(
        MyAnimals::NumberTuple(NumberTuple(-4, -2))
            .encode()
            .unwrap(),
        0b1111110011111100111111100000000000000000000000000000000000000000_u64 as i64
    );
    assert_eq!(
        MyAnimals::decode(
            0b1111110011111100111111100000000000000000000000000000000000000000_u64 as i64
        )
        .unwrap(),
        MyAnimals::NumberTuple(NumberTuple(-4, -2)),
    );
}
