type ClockCharacter<'a> = [&'a str; 10];

#[rustfmt::skip]
const ZERO: ClockCharacter = [
    "00000000",
    "00000000",
    "000  000",
    "000  000",
    "000  000",
    "000  000",
    "000  000",
    "000  000",
    "00000000",
    "00000000",
];

#[rustfmt::skip]
const ONE: ClockCharacter = [
    "111111  ",
    "111111  ",
    "   111  ",
    "   111  ",
    "   111  ",
    "   111  ",
    "   111  ",
    "   111  ",
    "11111111",
    "11111111",
];

#[rustfmt::skip]
const TWO: ClockCharacter = [
    "22222222",
    "22222222",
    "     222",
    "     222",
    "22222222",
    "22222222",
    "222     ",
    "222     ",
    "22222222",
    "22222222",
];

#[rustfmt::skip]
const THREE: ClockCharacter = [
    "33333333",
    "33333333",
    "     333",
    "     333",
    "33333333",
    "33333333",
    "     333",
    "     333",
    "33333333",
    "33333333",
];

#[rustfmt::skip]
const FOUR: ClockCharacter = [
    "444  444",
    "444  444",
    "444  444",
    "444  444",
    "44444444",
    "44444444",
    "     444",
    "     444",
    "     444",
    "     444",
];

#[rustfmt::skip]
const FIVE: ClockCharacter = [
    "55555555",
    "55555555",
    "555     ",
    "555     ",
    "55555555",
    "55555555",
    "     555",
    "     555",
    "55555555",
    "55555555",
];

#[rustfmt::skip]
const SIX: ClockCharacter = [
    "66666666",
    "66666666",
    "666     ",
    "666     ",
    "66666666",
    "66666666",
    "666  666",
    "666  666",
    "55555555",
    "55555555",
];

#[rustfmt::skip]
const SEVEN: ClockCharacter = [
    "77777777",
    "77777777",
    "     777",
    "     777",
    "     777",
    "     777",
    "     777",
    "     777",
    "     777",
    "     777",
];

#[rustfmt::skip]
const EIGHT: ClockCharacter = [
    "88888888",
    "88888888",
    "888  888",
    "888  888",
    "88888888",
    "88888888",
    "888  888",
    "888  888",
    "88888888",
    "88888888",
];

#[rustfmt::skip]
const NINE: ClockCharacter = [
    "99999999",
    "99999999",
    "999  999",
    "999  999",
    "99999999",
    "99999999",
    "     999",
    "     999",
    "     999",
    "     999",
];

pub const CLOCK_NUMBERS: [ClockCharacter; 10] =
    [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

#[rustfmt::skip]
pub const COLON: ClockCharacter = [
    "        ",
    "        ",
    "   ::   ",
    "   ::   ",
    "        ",
    "        ",
    "   ::   ",
    "   ::   ",
    "        ",
    "        "
];
