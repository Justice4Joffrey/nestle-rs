use nestle::derives::Nestle;
use proptest::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Nestle)]
#[repr(i8)]
pub enum LargeNumbersEnum {
    Number1 = -128,
    Number2 = -127,
    Number3 = -126,
    Number4 = -125,
    Number5 = -124,
    Number6 = -123,
    Number7 = -122,
    Number8 = -121,
    Number9 = -120,
    Number10 = -119,
    Number11 = -118,
    Number12 = -117,
    Number13 = -116,
    Number14 = -115,
    Number15 = -114,
    Number16 = -113,
    Number17 = -112,
    Number18 = -111,
    Number19 = -110,
    Number20 = -109,
    Number21 = -108,
    Number22 = -107,
    Number23 = -106,
    Number24 = -105,
    Number25 = -104,
    Number26 = -103,
    Number27 = -102,
    Number28 = -101,
    Number29 = -100,
    Number30 = -99,
    Number31 = -98,
    Number32 = -97,
    Number33 = -96,
    Number34 = -95,
    Number35 = -94,
    Number36 = -93,
    Number37 = -92,
    Number38 = -91,
    Number39 = -90,
    Number40 = -89,
    Number41 = -88,
    Number42 = -87,
    Number43 = -86,
    Number44 = -85,
    Number45 = -84,
    Number46 = -83,
    Number47 = -82,
    Number48 = -81,
    Number49 = -80,
    Number50 = -79,
    Number51 = -78,
    Number52 = -77,
    Number53 = -76,
    Number54 = -75,
    Number55 = -74,
    Number56 = -73,
    Number57 = -72,
    Number58 = -71,
    Number59 = -70,
    Number60 = -69,
    Number61 = -68,
    Number62 = -67,
    Number63 = -66,
    Number64 = -65,
    Number65 = -64,
    Number66 = -63,
    Number67 = -62,
    Number68 = -61,
    Number69 = -60,
    Number70 = -59,
    Number71 = -58,
    Number72 = -57,
    Number73 = -56,
    Number74 = -55,
    Number75 = -54,
    Number76 = -53,
    Number77 = -52,
    Number78 = -51,
    Number79 = -50,
    Number80 = -49,
    Number81 = -48,
    Number82 = -47,
    Number83 = -46,
    Number84 = -45,
    Number85 = -44,
    Number86 = -43,
    Number87 = -42,
    Number88 = -41,
    Number89 = -40,
    Number90 = -39,
    Number91 = -38,
    Number92 = -37,
    Number93 = -36,
    Number94 = -35,
    Number95 = -34,
    Number96 = -33,
    Number97 = -32,
    Number98 = -31,
    Number99 = -30,
    Number100 = -29,
    Number101 = -28,
    Number102 = -27,
    Number103 = -26,
    Number104 = -25,
    Number105 = -24,
    Number106 = -23,
    Number107 = -22,
    Number108 = -21,
    Number109 = -20,
    Number110 = -19,
    Number111 = -18,
    Number112 = -17,
    Number113 = -16,
    Number114 = -15,
    Number115 = -14,
    Number116 = -13,
    Number117 = -12,
    Number118 = -11,
    Number119 = -10,
    Number120 = -9,
    Number121 = -8,
    Number122 = -7,
    Number123 = -6,
    Number124 = -5,
    Number125 = -4,
    Number126 = -3,
    Number127 = -2,
    Number128 = -1,
    Number129 = 0,
    Number130 = 1,
    Number131 = 2,
    Number132 = 3,
    Number133 = 4,
    Number134 = 5,
    Number135 = 6,
    Number136 = 7,
    Number137 = 8,
    Number138 = 9,
    Number139 = 10,
    Number140 = 11,
    Number141 = 12,
    Number142 = 13,
    Number143 = 14,
    Number144 = 15,
    Number145 = 16,
    Number146 = 17,
    Number147 = 18,
    Number148 = 19,
    Number149 = 20,
    Number150 = 21,
    Number151 = 22,
    Number152 = 23,
    Number153 = 24,
    Number154 = 25,
    Number155 = 26,
    Number156 = 27,
    Number157 = 28,
    Number158 = 29,
    Number159 = 30,
    Number160 = 31,
    Number161 = 32,
    Number162 = 33,
    Number163 = 34,
    Number164 = 35,
    Number165 = 36,
    Number166 = 37,
    Number167 = 38,
    Number168 = 39,
    Number169 = 40,
    Number170 = 41,
    Number171 = 42,
    Number172 = 43,
    Number173 = 44,
    Number174 = 45,
    Number175 = 46,
    Number176 = 47,
    Number177 = 48,
    Number178 = 49,
    Number179 = 50,
    Number180 = 51,
    Number181 = 52,
    Number182 = 53,
    Number183 = 54,
    Number184 = 55,
    Number185 = 56,
    Number186 = 57,
    Number187 = 58,
    Number188 = 59,
    Number189 = 60,
    Number190 = 61,
    Number191 = 62,
    Number192 = 63,
    Number193 = 64,
    Number194 = 65,
    Number195 = 66,
    Number196 = 67,
    Number197 = 68,
    Number198 = 69,
    Number199 = 70,
    Number200 = 71,
    Number201 = 72,
    Number202 = 73,
    Number203 = 74,
    Number204 = 75,
    Number205 = 76,
    Number206 = 77,
    Number207 = 78,
    Number208 = 79,
    Number209 = 80,
    Number210 = 81,
    Number211 = 82,
    Number212 = 83,
    Number213 = 84,
    Number214 = 85,
    Number215 = 86,
    Number216 = 87,
    Number217 = 88,
    Number218 = 89,
    Number219 = 90,
    Number220 = 91,
    Number221 = 92,
    Number222 = 93,
    Number223 = 94,
    Number224 = 95,
    Number225 = 96,
    Number226 = 97,
    Number227 = 98,
    Number228 = 99,
    Number229 = 100,
    Number230 = 101,
    Number231 = 102,
    Number232 = 103,
    Number233 = 104,
    Number234 = 105,
    Number235 = 106,
    Number236 = 107,
    Number237 = 108,
    Number238 = 109,
    Number239 = 110,
    Number240 = 111,
    Number241 = 112,
    Number242 = 113,
    Number243 = 114,
    Number244 = 115,
    Number245 = 116,
    Number246 = 117,
    Number247 = 118,
    Number248 = 119,
    Number249 = 120,
    Number250 = 121,
    Number251 = 122,
    Number252 = 123,
    Number253 = 124,
    Number254 = 125,
    Number255 = 126,
    Number256 = 127,
}

#[allow(dead_code)]
pub fn large_enum_strategy() -> impl Strategy<Value = LargeNumbersEnum> {
    prop_oneof![
        Just(LargeNumbersEnum::Number1),
        Just(LargeNumbersEnum::Number2),
        Just(LargeNumbersEnum::Number3),
        Just(LargeNumbersEnum::Number4),
        Just(LargeNumbersEnum::Number5),
        Just(LargeNumbersEnum::Number6),
        Just(LargeNumbersEnum::Number7),
        Just(LargeNumbersEnum::Number8),
        Just(LargeNumbersEnum::Number9),
        Just(LargeNumbersEnum::Number10),
        Just(LargeNumbersEnum::Number11),
        Just(LargeNumbersEnum::Number12),
        Just(LargeNumbersEnum::Number13),
        Just(LargeNumbersEnum::Number14),
        Just(LargeNumbersEnum::Number15),
        Just(LargeNumbersEnum::Number16),
        Just(LargeNumbersEnum::Number17),
        Just(LargeNumbersEnum::Number18),
        Just(LargeNumbersEnum::Number19),
        Just(LargeNumbersEnum::Number20),
        Just(LargeNumbersEnum::Number21),
        Just(LargeNumbersEnum::Number22),
        Just(LargeNumbersEnum::Number23),
        Just(LargeNumbersEnum::Number24),
        Just(LargeNumbersEnum::Number25),
        Just(LargeNumbersEnum::Number26),
        Just(LargeNumbersEnum::Number27),
        Just(LargeNumbersEnum::Number28),
        Just(LargeNumbersEnum::Number29),
        Just(LargeNumbersEnum::Number30),
        Just(LargeNumbersEnum::Number31),
        Just(LargeNumbersEnum::Number32),
        Just(LargeNumbersEnum::Number33),
        Just(LargeNumbersEnum::Number34),
        Just(LargeNumbersEnum::Number35),
        Just(LargeNumbersEnum::Number36),
        Just(LargeNumbersEnum::Number37),
        Just(LargeNumbersEnum::Number38),
        Just(LargeNumbersEnum::Number39),
        Just(LargeNumbersEnum::Number40),
        Just(LargeNumbersEnum::Number41),
        Just(LargeNumbersEnum::Number42),
        Just(LargeNumbersEnum::Number43),
        Just(LargeNumbersEnum::Number44),
        Just(LargeNumbersEnum::Number45),
        Just(LargeNumbersEnum::Number46),
        Just(LargeNumbersEnum::Number47),
        Just(LargeNumbersEnum::Number48),
        Just(LargeNumbersEnum::Number49),
        Just(LargeNumbersEnum::Number50),
        Just(LargeNumbersEnum::Number51),
        Just(LargeNumbersEnum::Number52),
        Just(LargeNumbersEnum::Number53),
        Just(LargeNumbersEnum::Number54),
        Just(LargeNumbersEnum::Number55),
        Just(LargeNumbersEnum::Number56),
        Just(LargeNumbersEnum::Number57),
        Just(LargeNumbersEnum::Number58),
        Just(LargeNumbersEnum::Number59),
        Just(LargeNumbersEnum::Number60),
        Just(LargeNumbersEnum::Number61),
        Just(LargeNumbersEnum::Number62),
        Just(LargeNumbersEnum::Number63),
        Just(LargeNumbersEnum::Number64),
        Just(LargeNumbersEnum::Number65),
        Just(LargeNumbersEnum::Number66),
        Just(LargeNumbersEnum::Number67),
        Just(LargeNumbersEnum::Number68),
        Just(LargeNumbersEnum::Number69),
        Just(LargeNumbersEnum::Number70),
        Just(LargeNumbersEnum::Number71),
        Just(LargeNumbersEnum::Number72),
        Just(LargeNumbersEnum::Number73),
        Just(LargeNumbersEnum::Number74),
        Just(LargeNumbersEnum::Number75),
        Just(LargeNumbersEnum::Number76),
        Just(LargeNumbersEnum::Number77),
        Just(LargeNumbersEnum::Number78),
        Just(LargeNumbersEnum::Number79),
        Just(LargeNumbersEnum::Number80),
        Just(LargeNumbersEnum::Number81),
        Just(LargeNumbersEnum::Number82),
        Just(LargeNumbersEnum::Number83),
        Just(LargeNumbersEnum::Number84),
        Just(LargeNumbersEnum::Number85),
        Just(LargeNumbersEnum::Number86),
        Just(LargeNumbersEnum::Number87),
        Just(LargeNumbersEnum::Number88),
        Just(LargeNumbersEnum::Number89),
        Just(LargeNumbersEnum::Number90),
        Just(LargeNumbersEnum::Number91),
        Just(LargeNumbersEnum::Number92),
        Just(LargeNumbersEnum::Number93),
        Just(LargeNumbersEnum::Number94),
        Just(LargeNumbersEnum::Number95),
        Just(LargeNumbersEnum::Number96),
        Just(LargeNumbersEnum::Number97),
        Just(LargeNumbersEnum::Number98),
        Just(LargeNumbersEnum::Number99),
        Just(LargeNumbersEnum::Number100),
        Just(LargeNumbersEnum::Number101),
        Just(LargeNumbersEnum::Number102),
        Just(LargeNumbersEnum::Number103),
        Just(LargeNumbersEnum::Number104),
        Just(LargeNumbersEnum::Number105),
        Just(LargeNumbersEnum::Number106),
        Just(LargeNumbersEnum::Number107),
        Just(LargeNumbersEnum::Number108),
        Just(LargeNumbersEnum::Number109),
        Just(LargeNumbersEnum::Number110),
        Just(LargeNumbersEnum::Number111),
        Just(LargeNumbersEnum::Number112),
        Just(LargeNumbersEnum::Number113),
        Just(LargeNumbersEnum::Number114),
        Just(LargeNumbersEnum::Number115),
        Just(LargeNumbersEnum::Number116),
        Just(LargeNumbersEnum::Number117),
        Just(LargeNumbersEnum::Number118),
        Just(LargeNumbersEnum::Number119),
        Just(LargeNumbersEnum::Number120),
        Just(LargeNumbersEnum::Number121),
        Just(LargeNumbersEnum::Number122),
        Just(LargeNumbersEnum::Number123),
        Just(LargeNumbersEnum::Number124),
        Just(LargeNumbersEnum::Number125),
        Just(LargeNumbersEnum::Number126),
        Just(LargeNumbersEnum::Number127),
        Just(LargeNumbersEnum::Number128),
        Just(LargeNumbersEnum::Number129),
        Just(LargeNumbersEnum::Number130),
        Just(LargeNumbersEnum::Number131),
        Just(LargeNumbersEnum::Number132),
        Just(LargeNumbersEnum::Number133),
        Just(LargeNumbersEnum::Number134),
        Just(LargeNumbersEnum::Number135),
        Just(LargeNumbersEnum::Number136),
        Just(LargeNumbersEnum::Number137),
        Just(LargeNumbersEnum::Number138),
        Just(LargeNumbersEnum::Number139),
        Just(LargeNumbersEnum::Number140),
        Just(LargeNumbersEnum::Number141),
        Just(LargeNumbersEnum::Number142),
        Just(LargeNumbersEnum::Number143),
        Just(LargeNumbersEnum::Number144),
        Just(LargeNumbersEnum::Number145),
        Just(LargeNumbersEnum::Number146),
        Just(LargeNumbersEnum::Number147),
        Just(LargeNumbersEnum::Number148),
        Just(LargeNumbersEnum::Number149),
        Just(LargeNumbersEnum::Number150),
        Just(LargeNumbersEnum::Number151),
        Just(LargeNumbersEnum::Number152),
        Just(LargeNumbersEnum::Number153),
        Just(LargeNumbersEnum::Number154),
        Just(LargeNumbersEnum::Number155),
        Just(LargeNumbersEnum::Number156),
        Just(LargeNumbersEnum::Number157),
        Just(LargeNumbersEnum::Number158),
        Just(LargeNumbersEnum::Number159),
        Just(LargeNumbersEnum::Number160),
        Just(LargeNumbersEnum::Number161),
        Just(LargeNumbersEnum::Number162),
        Just(LargeNumbersEnum::Number163),
        Just(LargeNumbersEnum::Number164),
        Just(LargeNumbersEnum::Number165),
        Just(LargeNumbersEnum::Number166),
        Just(LargeNumbersEnum::Number167),
        Just(LargeNumbersEnum::Number168),
        Just(LargeNumbersEnum::Number169),
        Just(LargeNumbersEnum::Number170),
        Just(LargeNumbersEnum::Number171),
        Just(LargeNumbersEnum::Number172),
        Just(LargeNumbersEnum::Number173),
        Just(LargeNumbersEnum::Number174),
        Just(LargeNumbersEnum::Number175),
        Just(LargeNumbersEnum::Number176),
        Just(LargeNumbersEnum::Number177),
        Just(LargeNumbersEnum::Number178),
        Just(LargeNumbersEnum::Number179),
        Just(LargeNumbersEnum::Number180),
        Just(LargeNumbersEnum::Number181),
        Just(LargeNumbersEnum::Number182),
        Just(LargeNumbersEnum::Number183),
        Just(LargeNumbersEnum::Number184),
        Just(LargeNumbersEnum::Number185),
        Just(LargeNumbersEnum::Number186),
        Just(LargeNumbersEnum::Number187),
        Just(LargeNumbersEnum::Number188),
        Just(LargeNumbersEnum::Number189),
        Just(LargeNumbersEnum::Number190),
        Just(LargeNumbersEnum::Number191),
        Just(LargeNumbersEnum::Number192),
        Just(LargeNumbersEnum::Number193),
        Just(LargeNumbersEnum::Number194),
        Just(LargeNumbersEnum::Number195),
        Just(LargeNumbersEnum::Number196),
        Just(LargeNumbersEnum::Number197),
        Just(LargeNumbersEnum::Number198),
        Just(LargeNumbersEnum::Number199),
        Just(LargeNumbersEnum::Number200),
        Just(LargeNumbersEnum::Number201),
        Just(LargeNumbersEnum::Number202),
        Just(LargeNumbersEnum::Number203),
        Just(LargeNumbersEnum::Number204),
        Just(LargeNumbersEnum::Number205),
        Just(LargeNumbersEnum::Number206),
        Just(LargeNumbersEnum::Number207),
        Just(LargeNumbersEnum::Number208),
        Just(LargeNumbersEnum::Number209),
        Just(LargeNumbersEnum::Number210),
        Just(LargeNumbersEnum::Number211),
        Just(LargeNumbersEnum::Number212),
        Just(LargeNumbersEnum::Number213),
        Just(LargeNumbersEnum::Number214),
        Just(LargeNumbersEnum::Number215),
        Just(LargeNumbersEnum::Number216),
        Just(LargeNumbersEnum::Number217),
        Just(LargeNumbersEnum::Number218),
        Just(LargeNumbersEnum::Number219),
        Just(LargeNumbersEnum::Number220),
        Just(LargeNumbersEnum::Number221),
        Just(LargeNumbersEnum::Number222),
        Just(LargeNumbersEnum::Number223),
        Just(LargeNumbersEnum::Number224),
        Just(LargeNumbersEnum::Number225),
        Just(LargeNumbersEnum::Number226),
        Just(LargeNumbersEnum::Number227),
        Just(LargeNumbersEnum::Number228),
        Just(LargeNumbersEnum::Number229),
        Just(LargeNumbersEnum::Number230),
        Just(LargeNumbersEnum::Number231),
        Just(LargeNumbersEnum::Number232),
        Just(LargeNumbersEnum::Number233),
        Just(LargeNumbersEnum::Number234),
        Just(LargeNumbersEnum::Number235),
        Just(LargeNumbersEnum::Number236),
        Just(LargeNumbersEnum::Number237),
        Just(LargeNumbersEnum::Number238),
        Just(LargeNumbersEnum::Number239),
        Just(LargeNumbersEnum::Number240),
        Just(LargeNumbersEnum::Number241),
        Just(LargeNumbersEnum::Number242),
        Just(LargeNumbersEnum::Number243),
        Just(LargeNumbersEnum::Number244),
        Just(LargeNumbersEnum::Number245),
        Just(LargeNumbersEnum::Number246),
        Just(LargeNumbersEnum::Number247),
        Just(LargeNumbersEnum::Number248),
        Just(LargeNumbersEnum::Number249),
        Just(LargeNumbersEnum::Number250),
        Just(LargeNumbersEnum::Number251),
        Just(LargeNumbersEnum::Number252),
        Just(LargeNumbersEnum::Number253),
        Just(LargeNumbersEnum::Number254),
        Just(LargeNumbersEnum::Number255),
        Just(LargeNumbersEnum::Number256),
    ]
}
