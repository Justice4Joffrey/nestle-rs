use nestle::derives::Nestle;

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[repr(i8)]
pub enum Birds {
    Eagle = 1,
    Albatross = 2,
    Hawk = 3,
    Pigeon = -1,
    Dove = -2,
    Swallow = -3,
}

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[repr(i8)]
pub enum Mammals {
    Cat = 1,
    Dog = 2,
    Hamster = 3,
    Horse = -1,
    Pig = -2,
    Donkey = -3,
}

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[repr(i16)]
pub enum Fish {
    Shark = 1,
    Tuna = 2,
    Salmon = -1,
    Nemo = -2,
}

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[repr(i8)]
pub enum BirdNames {
    Donald = 1,
    Daffy = 2,
    Daisy = 3,
    Tweety = -1,
    Woody = -2,
    Zazu = -3,
}

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 16)]
pub struct MyBirds {
    pub name: BirdNames,
    pub bird: Birds,
}

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 16)]
pub struct MyBirdsTuple(pub BirdNames, pub Birds);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 16)]
pub struct NumberTuple(pub i8, pub i8);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 24)]
#[repr(i8)]
pub enum MyAnimals {
    Birds(MyBirds) = 1,
    Birds2(MyBirdsTuple) = 2,
    Mammals(Mammals) = -1,
    Fish(Fish) = -2,
    Number(i8) = -3,
    NumberTuple(NumberTuple) = -4,
}
