#![allow(non_upper_case_globals)]
#![allow(dead_code)]

///! fff-lang
///!
///! some error strings

pub const UnexpectedNonASCIIChar: &'static str = "Unexpected non ASCII char";
pub const InvalidNumericLiteral: &'static str = "Invalid numeric literal";
pub const NumLitShouldNotStartWith0: &'static str = "numeric literal should not start with 0";
pub const CStyleOctNumLitHelp: &'static str = "If you mean C style octal numeric literal, use `0o777` syntax";
pub const IntegralPrefixIsLowerCase: &'static str = "Integral prefix `0b`, `0o`, `0d`, `0x` is lower case";
pub const UnexpectedEOFInMaybeUnsignedIntPostfix: &'static str = "unexpected EOF in maybe unsigned integral postfix";
pub const UnexpectedEOFInMaybeSignedIntPostfix: &'static str = "unexpected EOF in maybe signed integral postfix";
pub const UnexpectedEOFInMaybeFloatingPostfix: &'static str = "unexpected EOF in maybe floating point postfix";
pub const UnexpectedEOFInExponent: &'static str = "unexpected EOF in floating point Exponent";
pub const UnexpectedValueAfterMaybeUnsignedIntPostfix: &'static str = "unexpected value after maybe unsigned integral postfix";
pub const UnexpectedValueAfterMaybeSignedIntPostfix: &'static str = "unexpected value after maybe signed integral postfix";
pub const UnexpectedValueAfterMaybeFloatingPostfix: &'static str = "unexpected value after maybe floating point postfix";
pub const UnexpectedNotEOF: &'static str = "unexpected not EOF";
pub const FloatPointUnderflow: &'static str = "floating point underflow";
pub const FloatPointOverflow: &'static str = "floating point overflow";
pub const IntegralOverflow: &'static str = "integral overflow";
pub const IntegralUnderflow: &'static str = "integral underflow";
pub const IntegralOverflowHelpMaxValue: [&'static str; 8] = [
    "Max value of i8 is 127",
    "Max value of u8 is 255",
    "Max value of i16 is 32767",
    "Max value of u16 is 65535",
    "Max value of i32 is 2147483647",
    "Max value of u32 is 4294967293",
    "Max value of i32 is 9223372036854775807",
    "Max value of u64 is 18446744073709551615",
];
pub const IntegralUnderflowHelpMinValue: [&'static str; 4] = [
    "Max value of i8 is -128",
    "Max value of i16 is -32768",
    "Max value of i32 is -2147483648",
    "Max value of i32 is -9223372036854775808",
];
pub const InvalidCharInFloatLiteral: &'static str = "invalid char in floating point literal";
pub const InvalidCharInIntLiteral: &'static str = "invalid char in integral literal";
pub const InvalidChar: &'static str = "invalid char";
pub const IntLiteralAllowedChars: [&'static str; 4] = [
    "It is a binary literal and only allows 0 and 1",
    "It is an octal literal and only allows 0-7",
    "It is a decimal literal and only allows 0-9",
    "It is a hexadecimal liteteral and only allows 0-9a-fA-F"
];
pub const InternalErrorAt: &'static str = "internal error at ";
pub const ExponentInIntLiteral: &'static str = "Exponent not allowed in integral literal";
pub const EmptyIntLiteral: &'static str = "empty integral literal";
pub const EmptyLiteral: &'static str = "empty literal";
pub const AndFloatPostfixInIntLiteral: &'static str = "And floating point literal not allowed in integral literal";
pub const DotInIntLiteral: &'static str = "decimal dot not allowed in integral literal";
pub const FloatExponentFloat: &'static str = "floating point exponentail should be integer";
pub const FloatPointOverflowHelpMaxValue: [&'static str; 4] = [
    "It is a single precision floating point literal and positive max value is about 3.40282347E+38",
    "It is a single precision floating point literal and negative max value is about -3.40282347E+38",
    "It is a double precision floating point literal and positive max value is about 1.7976931348623157E+308", // 1.79E308?
    "It is a double precision floating point literal and negative max value is about -1.7976931348623157E+308", // -1.79E308
];
pub const FloatPointUnderflowHelpMinValue: [&'static str; 4] = [
    "It is a single precision floating point literal and positive min value is about 1.17549435E-38",
    "It is a single precision floating point literal and negative min value is about -1.17549435E-38",
    "It is a double precision floating point literal and positive min value is about 2.2250738585072014E-308", // 1.79E-308?
    "It is a double precision floating point literal and negative min value is about -2.2250738585072014E-308", // -1.79E-308?
];
pub const NegativeOperatorOnUnsignedInt: &'static str = "Negative operator should not apply to unsigned integral literal";
pub const UnderscoreDouble: &'static str = "continuous underscore not allowed";
pub const UnderscoreArroundDot: &'static str = "underscore should not before or after decimal dot";
pub const UnderscoreAtHead: &'static str = "underscore should not be before first digit of numeric literal";
pub const UnderscoreAtExponentHead: &'static str = "underscore should not be before first digit of floating point literal exponent";
pub const UnderscoreAtEnd: &'static str = "underscore should not be end of numeric literal";
pub const UnderscoreInMaybeSignedIntPostfix: &'static str = "underscore should not be within maybe signed integral postfix";
pub const UnderscoreInMaybeUnsignedIntPostfix: &'static str = "underscore should not be within maybe unsigned integral postfix";
pub const UnderscoreInMaybeFloatPointPostfix: &'static str = "underscore should not be within maybe floating point integral postfix";
pub const DotAtHead: &'static str = "decimal dot should not be before first digit of numeric literal";
pub const DotAtEnd: &'static str = "decimal dot should not be after last digit of numeric literal";
pub const DotDouble: &'static str = "multiple decimal dot";
pub const MaybeIntPostfixInFloatPoint: &'static str = "maybe integral postfix not allowed in floating point literal";
pub const UseReservedKeyword: &str = "Use of reserved keyword";