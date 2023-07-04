use nestle::core::Nestle;
use proptest::prelude::*;

use crate::common::*;

mod common;

fn birds_strategy() -> impl Strategy<Value = Birds> {
    prop_oneof![
        Just(Birds::Eagle),
        Just(Birds::Albatross),
        Just(Birds::Hawk),
        Just(Birds::Pigeon),
        Just(Birds::Dove),
        Just(Birds::Swallow),
    ]
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
        Just(BirdNames::Donald),
        Just(BirdNames::Daffy),
        Just(BirdNames::Daisy),
        Just(BirdNames::Tweety),
        Just(BirdNames::Woody),
        Just(BirdNames::Zazu),
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
        my_birds_strategy().prop_map(MyAnimals::Birds),
        my_birds_tuple_strategy().prop_map(MyAnimals::Birds2),
        mammals_strategy().prop_map(MyAnimals::Mammals),
        fish_strategy().prop_map(MyAnimals::Fish),
        any::<i8>().prop_map(MyAnimals::Number),
        (any::<i8>(), any::<i8>())
            .prop_map(|(first, second)| NumberTuple(first, second))
            .prop_map(MyAnimals::NumberTuple),
    ]
}

proptest! {
    #[test]
    fn parses_birds(bird in birds_strategy()) {
        let encoded = bird.encode().unwrap();
        prop_assert_eq!(bird, Nestle::decode(encoded).unwrap());
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
}
