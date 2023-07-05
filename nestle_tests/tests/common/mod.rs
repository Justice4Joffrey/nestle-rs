use nestle::derives::Nestle;
pub use large_enum::*;

mod large_enum;

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[repr(i8)]
pub enum Birds {
    AndeanCondor = 127,
    Eagle = 1,
    Albatross = 2,
    Hawk = 3,
    Pigeon = -1,
    Dove = -2,
    Swallow = -3,
    Hummingbird = -128,
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
    Ace = 127,
    Donald = 1,
    Daffy = 2,
    Daisy = 3,
    Tweety = -1,
    Woody = -2,
    Zazu = -3,
    Zippy = -128,
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
    LargeEnum1(LargeNumbersEnum) = 3,
    Birds(MyBirds) = 1,
    Birds2(MyBirdsTuple) = 2,
    Mammals(Mammals) = -1,
    Fish(Fish) = -2,
    Number(i8) = -3,
    NumberTuple(NumberTuple) = -4,
    LargeEnum2(LargeNumbersEnum) = -5,
}

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 64)]
pub struct Full64Numbers(pub u32, pub i16, pub i8, pub u8);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 56)]
pub struct FullByteNumbers(pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 56)]
pub struct FullDifferentNumbers(pub i32, pub i16, pub i8);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 56)]
pub struct FullSignedUnsignedNumbers(pub u32, pub i8, pub u16);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 56)]
pub struct FullTupleWithNumbers(pub NumberTuple, pub u16, pub u8, pub u16);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 48)]
pub struct NotFullByteNumbers(pub i8, pub i8, pub u8, pub i8, pub u8, pub u8);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 40)]
pub struct NotFullDifferentNumbers(pub i32, pub i8);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 48)]
pub struct NotFullSignedUnsignedNumbers(pub u16, pub i32);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 16)]
pub struct DoubleUnsignedEightNumbers(pub u8, pub u8);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 40)]
pub struct NotFullTuplesWithNumbers(pub DoubleUnsignedEightNumbers, pub u16, pub u8);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 56)]
pub struct FullBool(pub bool, pub bool, pub bool, pub bool, pub bool, pub bool, pub bool);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 24)]
pub struct NotFullBool(pub bool, pub bool, pub bool);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 56)]
pub struct BoolWithNumbers(pub bool, pub u32, pub bool, pub i8);

#[derive(Clone, Debug, PartialEq, Nestle)]
#[nestle(width = 48)]
pub struct FloatWithOthers(pub u8, pub f32, pub bool);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 0)]
pub struct Empty();

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 0)]
pub struct ALotOfUnits(pub (), pub (), pub (), pub (), pub (), pub (), pub (), pub (), pub (), pub ());

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 40)]
pub struct UnitsWithOthers(pub (), pub (), pub bool, pub (), pub u8, pub (), pub i8, pub (), pub i16);

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 56)]
pub struct CharWithOthers(pub u8, pub char, pub i16, pub ());

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 16)]
pub struct SinglePrimitiveFieldStruct {
    pub field: u16
}

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 56)]
pub struct SeveralPrimitiveFieldsStruct {
    pub field1: char,
    pub field2: u16,
    pub field3: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 56)]
pub struct SingleTupleFieldStruct {
    pub field: CharWithOthers
}

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 56)]
pub struct SingleStructFieldStruct {
    pub field: SingleTupleFieldStruct
}

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[nestle(width = 48)]
pub struct SeveralDifferentFieldsStruct {
    pub field1: BirdNames,
    pub field2: u8,
    pub field3: ALotOfUnits,
    pub field4: SinglePrimitiveFieldStruct,
    pub field5: Empty,
    pub field6: DoubleUnsignedEightNumbers,
}

#[derive(Clone, Debug, PartialEq, Nestle)]
#[nestle(width = 64)]
#[repr(i8)]
pub enum DiverseZoo {
    FullByteNumbers(FullByteNumbers) = -128,
    FullDifferentNumbers(FullDifferentNumbers) = -126,
    FullSignedUnsignedNumbers(FullSignedUnsignedNumbers) = -125,
    FullTuplesWithNumbers(FullTupleWithNumbers) = -124,
    NotFullByteNumbers(NotFullByteNumbers) = -123,
    NotFullDifferentNumbers(NotFullDifferentNumbers) = -122,
    NotFullSignedUnsignedNumbers(NotFullSignedUnsignedNumbers) = -121,
    NotFullTuplesWithNumbers(NotFullTuplesWithNumbers) = -120,
    NotFullSingleTupleWithNumbers(DoubleUnsignedEightNumbers) = -119,
    NotFullTupleWithSingleNumber(u32) = -118,
    FullBool(FullBool) = -117,
    NotFullBool(NotFullBool) = -116,
    BoolWithNumbers(BoolWithNumbers) = -115,
    SingleBool(bool) = -114,
    FloatWithOthers(FloatWithOthers) = -113,
    Empty(Empty) = -112,
    ALotOfUnits(ALotOfUnits) = -111,
    UnitsWithOthers(UnitsWithOthers) = -110,
    Char(CharWithOthers) = -109,
    SinglePrimitiveFieldStruct(SinglePrimitiveFieldStruct) = -108,
    SingleTupleFieldStruct(SingleTupleFieldStruct) = -1,
    SingleStructFieldStruct(SingleStructFieldStruct) = 0,
    SeveralPrimitiveFieldsStruct(SeveralPrimitiveFieldsStruct) = 1,
    LargeNumbersEnum(LargeNumbersEnum) = 2,
    SeveralDifferentFieldsStruct(SeveralDifferentFieldsStruct) = 127,
}
