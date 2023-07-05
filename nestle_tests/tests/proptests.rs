use nestle::core::Nestle;
use proptest::prelude::*;

use crate::common::*;

mod common;

fn birds_strategy() -> impl Strategy<Value = Birds> {
    prop_oneof![
        Just(Birds::AndeanCondor),
        Just(Birds::Eagle),
        Just(Birds::Albatross),
        Just(Birds::Hawk),
        Just(Birds::Pigeon),
        Just(Birds::Dove),
        Just(Birds::Swallow),
        Just(Birds::Hummingbird),
    ]
}

fn full_64_numbers_strategy() -> impl Strategy<Value = Full64Numbers> {
    (any::<u32>(), any::<i16>(), any::<i8>(), any::<u8>())
        .prop_map(|(first, second, third, fourth)| Full64Numbers(first, second, third, fourth))
}

fn full_bytes_number_strategy() -> impl Strategy<Value = FullByteNumbers> {
    (any::<i8>(), any::<i8>(), any::<i8>(), any::<i8>(), any::<i8>(), any::<i8>(), any::<i8>())
        .prop_map(|(first, second, third, fourth, fifth, sixth, seventh)| FullByteNumbers(first, second, third, fourth, fifth, sixth, seventh))
}

fn full_different_numbers_strategy() -> impl Strategy<Value = FullDifferentNumbers> {
    (any::<i32>(), any::<i16>(), any::<i8>())
        .prop_map(|(first, second, third)| FullDifferentNumbers(first, second, third))
}

fn full_signed_unsigned_numbers_strategy() -> impl Strategy<Value = FullSignedUnsignedNumbers> {
    (any::<u32>(), any::<i8>(), any::<u16>())
        .prop_map(|(first, second, third)| FullSignedUnsignedNumbers(first, second, third))
}

fn full_tuples_with_numbers_strategy() -> impl Strategy<Value = FullTupleWithNumbers> {
    (any::<i8>(), any::<i8>(), any::<u16>(), any::<u8>(), any::<u16>())
        .prop_map(|(first, second, third, fourth, fifth)| FullTupleWithNumbers(NumberTuple(first, second), third, fourth,fifth))
}

fn not_full_bytes_number_strategy() -> impl Strategy<Value = NotFullByteNumbers> {
    (any::<i8>(), any::<i8>(), any::<u8>(), any::<i8>(), any::<u8>(), any::<u8>())
        .prop_map(|(first, second, third, fourth, fifth, sixth)| NotFullByteNumbers(first, second, third, fourth, fifth, sixth))
}

fn not_full_different_numbers_strategy() -> impl Strategy<Value = NotFullDifferentNumbers> {
    (any::<i32>(), any::<i8>())
        .prop_map(|(first, second)| NotFullDifferentNumbers(first, second))
}

fn not_full_signed_unsigned_numbers_strategy() -> impl Strategy<Value = NotFullSignedUnsignedNumbers> {
    (any::<u16>(), any::<i32>())
        .prop_map(|(first, second)| NotFullSignedUnsignedNumbers(first, second))
}

fn double_unsigned_numbers_strategy() -> impl Strategy<Value = DoubleUnsignedEightNumbers> {
    (any::<u8>(), any::<u8>())
        .prop_map(|(first, second)| DoubleUnsignedEightNumbers(first, second))
}

fn not_full_tuples_with_numbers_strategy() -> impl Strategy<Value = NotFullTuplesWithNumbers> {
    (double_unsigned_numbers_strategy(), any::<u16>(), any::<u8>())
        .prop_map(|(first, second, third)| NotFullTuplesWithNumbers(first, second, third))
}

fn full_bool_strategy() -> impl Strategy<Value = FullBool> {
    (any::<bool>(), any::<bool>(), any::<bool>(), any::<bool>(), any::<bool>(), any::<bool>(), any::<bool>())
        .prop_map(|(first, second, third, fourth, fifth, sixth, seventh)| FullBool(first, second, third, fourth, fifth, sixth, seventh))
}

fn not_full_bool_strategy() -> impl Strategy<Value = NotFullBool> {
    (any::<bool>(), any::<bool>(), any::<bool>())
        .prop_map(|(first, second, third)| NotFullBool(first, second, third))
}

fn bool_with_numbers_strategy() -> impl Strategy<Value = BoolWithNumbers> {
    (any::<bool>(), any::<u32>(), any::<bool>(), any::<i8>())
        .prop_map(|(first, second, third, fourth)| BoolWithNumbers(first, second, third, fourth))
}

fn float_with_others_strategy() -> impl Strategy<Value = FloatWithOthers> {
    (any::<u8>(), any::<f32>(), any::<bool>())
        .prop_map(|(first, second, third)| FloatWithOthers(first, second, third))
}

fn empty_strategy() -> impl Strategy<Value = Empty> {
    Just(Empty())
}

fn a_lot_of_units_strategy() -> impl Strategy<Value = ALotOfUnits> {
    Just(ALotOfUnits((), (), (), (), (), (), (), (), (), ()))
}

fn units_with_others_strategy() -> impl Strategy<Value = UnitsWithOthers> {
    (any::<bool>(), any::<u8>(), any::<i8>(), any::<i16>())
        .prop_map(|(first, second, third, fourth)| UnitsWithOthers((), (), first, (), second, (), third, (), fourth))
}

fn char_with_others_strategy() -> impl Strategy<Value = CharWithOthers> {
    (any::<u8>(), any::<char>(), any::<i16>())
        .prop_map(|(first, second, third)| CharWithOthers(first, second, third, ()))
}

fn struct_single_primitive_field_strategy() -> impl Strategy<Value = SinglePrimitiveFieldStruct> {
    any::<u16>()
        .prop_map(|first| SinglePrimitiveFieldStruct { field: first })
}

fn struct_several_primitive_fields_strategy() -> impl Strategy<Value = SeveralPrimitiveFieldsStruct> {
    (any::<char>(), any::<u16>(), any::<bool>())
        .prop_map(|(first, second, third)| SeveralPrimitiveFieldsStruct { field1: first, field2: second, field3: third })
}

fn struct_single_tuple_field_strategy() -> impl Strategy<Value = SingleTupleFieldStruct> {
    char_with_others_strategy().prop_map(|value| SingleTupleFieldStruct { field: value })
}

fn struct_single_struct_field_strategy() -> impl Strategy<Value = SingleStructFieldStruct> {
    struct_single_tuple_field_strategy().prop_map(|value| SingleStructFieldStruct { field: value })
}

fn struct_several_different_fields_strategy() -> impl Strategy<Value = SeveralDifferentFieldsStruct> {
    (bird_names_strategy(), any::<u8>(), a_lot_of_units_strategy(), struct_single_primitive_field_strategy(), empty_strategy(), double_unsigned_numbers_strategy()).prop_map(|(first, second, third, fourth, fifth, sixth)| SeveralDifferentFieldsStruct {
        field1: first,
        field2: second,
        field3: third,
        field4: fourth,
        field5: fifth,
        field6: sixth,
    })
}



fn mammals_strategy() -> impl Strategy<Value = Mammals> {
    prop_oneof![
        Just(Mammals::Cat),
        Just(Mammals::Dog),
        Just(Mammals::Hamster),
        Just(Mammals::Horse),
        Just(Mammals::Pig),
        Just(Mammals::Donkey),
    ]
}

fn fish_strategy() -> impl Strategy<Value = Fish> {
    prop_oneof![
        Just(Fish::Shark),
        Just(Fish::Tuna),
        Just(Fish::Salmon),
        Just(Fish::Nemo),
    ]
}

fn bird_names_strategy() -> impl Strategy<Value = BirdNames> {
    prop_oneof![
        Just(BirdNames::Ace),
        Just(BirdNames::Donald),
        Just(BirdNames::Daffy),
        Just(BirdNames::Daisy),
        Just(BirdNames::Tweety),
        Just(BirdNames::Woody),
        Just(BirdNames::Zazu),
        Just(BirdNames::Zippy),
    ]
}

fn diverse_zoo_strategy() -> impl Strategy<Value = DiverseZoo> {
    prop_oneof![
        full_bytes_number_strategy().prop_map(DiverseZoo::FullByteNumbers),
        full_different_numbers_strategy().prop_map(DiverseZoo::FullDifferentNumbers),
        full_signed_unsigned_numbers_strategy().prop_map(DiverseZoo::FullSignedUnsignedNumbers),
        full_tuples_with_numbers_strategy().prop_map(DiverseZoo::FullTuplesWithNumbers),
        not_full_bytes_number_strategy().prop_map(DiverseZoo::NotFullByteNumbers),
        not_full_different_numbers_strategy().prop_map(DiverseZoo::NotFullDifferentNumbers),
        not_full_signed_unsigned_numbers_strategy().prop_map(DiverseZoo::NotFullSignedUnsignedNumbers),
        not_full_tuples_with_numbers_strategy().prop_map(DiverseZoo::NotFullTuplesWithNumbers),
        double_unsigned_numbers_strategy().prop_map(DiverseZoo::NotFullSingleTupleWithNumbers),
        any::<u32>().prop_map(DiverseZoo::NotFullTupleWithSingleNumber),
        full_bool_strategy().prop_map(DiverseZoo::FullBool),
        not_full_bool_strategy().prop_map(DiverseZoo::NotFullBool),
        bool_with_numbers_strategy().prop_map(DiverseZoo::BoolWithNumbers),
        any::<bool>().prop_map(DiverseZoo::SingleBool),
        float_with_others_strategy().prop_map(DiverseZoo::FloatWithOthers),
        empty_strategy().prop_map(DiverseZoo::Empty),
        a_lot_of_units_strategy().prop_map(DiverseZoo::ALotOfUnits),
        units_with_others_strategy().prop_map(DiverseZoo::UnitsWithOthers),
        char_with_others_strategy().prop_map(DiverseZoo::Char),
        struct_single_primitive_field_strategy().prop_map(DiverseZoo::SinglePrimitiveFieldStruct),
        struct_single_tuple_field_strategy().prop_map(DiverseZoo::SingleTupleFieldStruct),
        struct_single_struct_field_strategy().prop_map(DiverseZoo::SingleStructFieldStruct),
        struct_several_primitive_fields_strategy().prop_map(DiverseZoo::SeveralPrimitiveFieldsStruct),
    ]
}

fn my_birds_strategy() -> impl Strategy<Value = MyBirds> {
    (bird_names_strategy(), birds_strategy()).prop_map(|(name, bird)| MyBirds { name, bird })
}

fn my_birds_tuple_strategy() -> impl Strategy<Value = MyBirdsTuple> {
    (bird_names_strategy(), birds_strategy()).prop_map(|(name, bird)| MyBirdsTuple(name, bird))
}

fn my_animals_strategy() -> impl Strategy<Value = MyAnimals> {
    prop_oneof![
        large_enum_strategy().prop_map(MyAnimals::LargeEnum1),
        my_birds_strategy().prop_map(MyAnimals::Birds),
        my_birds_tuple_strategy().prop_map(MyAnimals::Birds2),
        mammals_strategy().prop_map(MyAnimals::Mammals),
        fish_strategy().prop_map(MyAnimals::Fish),
        any::<i8>().prop_map(MyAnimals::Number),
        (any::<i8>(), any::<i8>())
            .prop_map(|(first, second)| NumberTuple(first, second))
            .prop_map(MyAnimals::NumberTuple),
        large_enum_strategy().prop_map(MyAnimals::LargeEnum2),
    ]
}

proptest! {
    #[test]
    fn parses_birds(bird in birds_strategy()) {
        let encoded = bird.encode().unwrap();
        prop_assert_eq!(bird, Nestle::decode(encoded).unwrap());
    }

    #[test]
    fn parses_large_enum(number in large_enum_strategy()) {
        let encoded = number.encode().unwrap();
        prop_assert_eq!(number, Nestle::decode(encoded).unwrap());
    }

    #[test]
    fn parses_mammals(mammal in mammals_strategy()) {
        let encoded = mammal.encode().unwrap();
        prop_assert_eq!(mammal, Nestle::decode(encoded).unwrap());
    }

    #[test]
    fn parses_fish(fish in fish_strategy()) {
        let encoded = fish.encode().unwrap();
        prop_assert_eq!(fish, Nestle::decode(encoded).unwrap());
    }

    #[test]
    fn parses_bird_names(bird in bird_names_strategy()) {
        let encoded = bird.encode().unwrap();
        prop_assert_eq!(bird, Nestle::decode(encoded).unwrap());
    }

    #[test]
    fn parses_my_bird(my_bird in my_birds_strategy()) {
        let encoded = my_bird.encode().unwrap();
        let decoded_bird: MyBirds = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(my_bird, decoded_bird);
    }

    #[test]
    fn parses_my_bird_tuple(my_bird in my_birds_tuple_strategy()) {
        let encoded = my_bird.encode().unwrap();
        let decoded_bird: MyBirdsTuple = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(my_bird, decoded_bird);
    }

    #[test]
    fn parses_number_tuple(first in any::<i8>(), second in any::<i8>()) {
        let number_tuple = NumberTuple(first, second);
        let encoded = number_tuple.encode().unwrap();
        let decoded_tuple: NumberTuple = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(number_tuple, decoded_tuple);
    }

    #[test]
    fn parses_my_animals(animal in my_animals_strategy()) {
        let encoded = animal.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(animal, decoded);
    }

    #[test]
    fn parses_full_byte_numbers(numbers in full_bytes_number_strategy()) {
        let encoded = numbers.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(numbers, decoded);
    }

    #[test]
    fn parses_full_64_numbers(numbers in full_64_numbers_strategy()) {
        let encoded = numbers.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(numbers, decoded);
    }

    #[test]
    fn parses_full_different_numbers(numbers in full_different_numbers_strategy()) {
        let encoded = numbers.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(numbers, decoded);
    }

    #[test]
    fn parses_full_signed_unsigned_numbers(numbers in full_signed_unsigned_numbers_strategy()) {
        let encoded = numbers.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(numbers, decoded);
    }

    #[test]
    fn parses_full_tuples_with_numbers(values in full_tuples_with_numbers_strategy()) {
        let encoded = values.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(values, decoded)
    }

    #[test]
    fn parses_not_full_byte_numbers(numbers in not_full_bytes_number_strategy()) {
        let encoded = numbers.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(numbers, decoded);
    }

    #[test]
    fn parses_not_full_different_numbers(numbers in not_full_different_numbers_strategy()) {
        let encoded = numbers.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(numbers, decoded);
    }

    #[test]
    fn parses_not_full_signed_unsigned_numbers(numbers in not_full_signed_unsigned_numbers_strategy()) {
        let encoded = numbers.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(numbers, decoded);
    }

    #[test]
    fn parses_not_full_tuples_with_numbers(value in not_full_tuples_with_numbers_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_full_bools(value in full_bool_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_not_full_bools(value in not_full_bool_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_bool_with_numbers(value in bool_with_numbers_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_float_with_others(value in float_with_others_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_empty(value in empty_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_a_lot_of_nits(value in a_lot_of_units_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_units_with_others(value in units_with_others_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_char_with_others(value in char_with_others_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_struct_single_primitive_field(value in struct_single_primitive_field_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_struct_several_primitive_fields(value in struct_several_primitive_fields_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_struct_single_tuple_field(value in struct_single_tuple_field_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_struct_single_struct_field(value in struct_single_struct_field_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_struct_several_different_fields(value in struct_several_different_fields_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn parses_diverse_zoo(value in diverse_zoo_strategy()) {
        let encoded = value.encode().unwrap();
        let decoded = Nestle::decode(encoded).unwrap();
        prop_assert_eq!(value, decoded);
    }
}
