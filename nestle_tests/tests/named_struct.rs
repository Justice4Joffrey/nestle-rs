use nestle::core::Nestle;

use crate::common::{BirdNames, Birds, MyBirds};

mod common;

#[test]
fn test_named_struct() {
    assert_eq!(
        MyBirds {
            name: BirdNames::Donald,
            bird: Birds::Eagle,
        }
        .encode_unchecked(),
        0b0000000100000001000000000000000000000000000000000000000000000000_u64 as i64
    );
    assert_eq!(
        MyBirds::decode(
            0b0000000100000001000000000000000000000000000000000000000000000000_u64 as i64
        )
        .unwrap(),
        MyBirds {
            name: BirdNames::Donald,
            bird: Birds::Eagle,
        }
    );
    assert_eq!(
        MyBirds {
            name: BirdNames::Tweety,
            bird: Birds::Pigeon,
        }
        .encode_unchecked(),
        0b1111111111111111000000000000000000000000000000000000000000000000_u64 as i64
    );
    assert_eq!(
        MyBirds::decode(
            0b1111111111111111000000000000000000000000000000000000000000000000_u64 as i64
        )
        .unwrap(),
        MyBirds {
            name: BirdNames::Tweety,
            bird: Birds::Pigeon,
        }
    );
}

#[test]
fn test_woody_pigeon() {
    let original = MyBirds {
        name: BirdNames::Woody,
        bird: Birds::Pigeon,
    };

    let encoded = original.encode_unchecked();
    let decoded: MyBirds = Nestle::decode(encoded).unwrap();
    assert_eq!(decoded, original);
}
