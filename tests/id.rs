#![allow(clippy::unusual_byte_groupings, clippy::inconsistent_digit_grouping)]
#![allow(clippy::zero_prefixed_literal)]
#![allow(clippy::mixed_case_hex_literals)]

use culit::culit;

#[test]
fn ui() {
    let trybuild = trybuild::TestCases::new();
    trybuild.compile_fail("tests/ui/*.rs");
}

#[test]
#[culit]
fn integer_literals() {
    // Binary no undescrore
    assert_eq!(0b1id, 1);
    assert_eq!(0b10id, 2);
    assert_eq!(0b100id, 4);
    assert_eq!(0b1111id, 15);

    // Binary with underscores
    assert_eq!(0b1_0_0id, 4);
    assert_eq!(0b_1_0_0id, 4);
    assert_eq!(0b1_111_000id, 120);
    assert_eq!(0b_1_111_000_id, 120);

    // Octal no underscore
    assert_eq!(0o7id, 7);
    assert_eq!(0o10id, 8);
    assert_eq!(0o100id, 64);
    assert_eq!(0o777id, 511);

    // Octal with underscores
    assert_eq!(0o1_0_0id, 64);
    assert_eq!(0o_1_0_0id, 64);
    assert_eq!(0o7_7_7id, 511);
    assert_eq!(0o_7_7_7_id, 511);

    // Float no underscore
    assert_eq!(0id, 0);
    assert_eq!(1id, 1);
    assert_eq!(10id, 10);
    assert_eq!(100id, 100);
    assert_eq!(999id, 999);

    // Float with underscores
    assert_eq!(1_0_0id, 100);
    assert_eq!(1_000id, 1000);
    assert_eq!(9_9_9id, 999);
    assert_eq!(1_000_000id, 1_000_000);
    assert_eq!(1_0_0_id, 100);

    // Float with leading zeros (still valid integer)
    assert_eq!(000id, 0);
    assert_eq!(0007id, 7);
    assert_eq!(0010id, 10);

    // Hexadecimal no undescore
    assert_eq!(0x1id, 1);
    assert_eq!(0x10id, 16);
    assert_eq!(0x100id, 256);
    assert_eq!(0xABCid, 0xABC);
    assert_eq!(0xabcid, 0xabc);
    assert_eq!(0xDEAD_BEEFid, 0xDEADBEEF);

    // Hexadecimal with underscores
    assert_eq!(0x1_0_0id, 256);
    assert_eq!(0x_1_0_0id, 256);
    assert_eq!(0xAB_CDid, 0xABCD);
    assert_eq!(0xA_B_CDid, 0xABCD);
    assert_eq!(0x_ABC_D_id, 0xABCD);

    // edge cases for 0
    assert_eq!(0b0id, 0);
    assert_eq!(0o0id, 0);
    assert_eq!(0x0id, 0);

    // Large numbers with separators
    assert_eq!(1_000_000_000id, 1_000_000_000);
    assert_eq!(0b1111_1111_1111_1111id, 0xFFFF);
    assert_eq!(0o7_7_7_7id, 0o7777);
    assert_eq!(0xFFFF_FFFFid, 0xFFFFFFFF);
}

#[test]
#[culit]
fn float() {
    // With fractional, no exponent
    assert_eq!(70.0id, 70.0);
    assert_eq!(70.8id, 70.8);
    assert_eq!(7_0.8id, 7_0.8);
    assert_eq!(7_0_.8id, 7_0_.8);

    // With exponent only
    assert_eq!(70e7id, 70e7);
    assert_eq!(70e-7id, 70e-7);
    assert_eq!(70e+7id, 70e+7);
    assert_eq!(7_0e7id, 7_0e7);
    assert_eq!(7_0_e7id, 7_0_e7);
    assert_eq!(7_0_e-7id, 7_0_e-7);

    // With fractional and exponent
    assert_eq!(70.8e7id, 70.8e7);
    assert_eq!(70.8e-7id, 70.8e-7);
    assert_eq!(70.8e+7id, 70.8e+7);
    assert_eq!(7_0.8e7id, 7_0.8e7);
    assert_eq!(7_0.8e-7id, 7_0.8e-7);
    assert_eq!(7_0.8e+7id, 7_0.8e+7);

    // Fractional only, with underscores
    assert_eq!(70.0id, 70.0);
    assert_eq!(70.000id, 70.000);
    assert_eq!(70.123id, 70.123);
    assert_eq!(7_0.1_2_3id, 7_0.1_2_3);
    assert_eq!(7_0_.0id, 7_0_.0);

    // Fractional + exponent, underscores everywhere
    assert_eq!(70.0e7id, 70.0e7);
    assert_eq!(70.0e-7id, 70.0e-7);
    assert_eq!(70.0e+7id, 70.0e+7);
    assert_eq!(70.123e7id, 70.123e7);
    assert_eq!(7_0.1_2_3e7id, 7_0.1_2_3e7);
    assert_eq!(7_0.1_2_3e-7id, 7_0.1_2_3e-7);
    assert_eq!(7_0_.1_2_3_e_7id, 7_0_.1_2_3_e_7);
    assert_eq!(7_0_.0_e_7id, 7_0_.0_e_7);
    assert_eq!(7_0_.0_e-7id, 7_0_.0_e-7);
    assert_eq!(7_0_.0_e+7id, 7_0_.0_e+7);

    // Small edge cases
    assert_eq!(0.0id, 0.0);
    assert_eq!(0e7id, 0e7);
    assert_eq!(0.007id, 0.007);
    assert_eq!(0.0e7id, 0.0e7);
    assert_eq!(0.123id, 0.123);
    assert_eq!(0.123e7id, 0.123e7);
    assert_eq!(0.123e-7id, 0.123e-7);
}

#[test]
#[culit]
fn str() {
    assert_eq!("foo"id, stringify!("foo"));
    assert_eq!("\nfoo"id, stringify!("\nfoo"));
    assert_eq!(r"foo"id, stringify!("foo"));
    assert_eq!(r#"foo"#id, stringify!("foo"));
    assert_eq!(r#"foo\"#id, stringify!("foo\\"));
}

#[test]
#[culit]
fn byte_char() {
    assert_eq!(b'a'id, b'a');
}

#[test]
#[culit]
fn byte_str() {
    assert_eq!(b"hello"id, stringify!(b"hello"));
    assert_eq!(b"hell\\o"id, stringify!(b"hell\\o"));
    assert_eq!(br"hell\o"id, stringify!(b"hell\\o"));
    assert_eq!(br#"hello"#id, stringify!(b"hello"));
}

mod local {
    use culit::culit;

    mod custom_literal {
        pub mod character {
            macro_rules! id {
                ($value:literal) => {
                    "a"
                };
            }
            pub(crate) use id;
        }
    }

    #[test]
    #[culit]
    fn char() {
        assert_eq!('a'id, stringify!('a'));
    }

    #[culit(local)]
    #[test]
    fn char_local() {
        assert_eq!('a'id, "a");
    }
}

#[test]
#[culit]
fn c_str() {
    assert_eq!(c"hello"id, stringify!(c"hello"));
    assert_eq!(c"hell\\o"id, stringify!(c"hell\\o"));
    assert_eq!(cr"hell\o"id, stringify!(c"hell\\o"));
    assert_eq!(cr#"hello"#id, stringify!(c"hello"));
}

mod custom_literal {
    // `id` for "Identity"
    macro_rules! id {
        ($($tt:tt)*) => {
            stringify!($($tt)*)
        };
    }

    pub(crate) use id;

    pub mod integer {
        macro_rules! id {
            ($value:literal) => {{
                $value as i64
            }};
        }
        pub(crate) use id;
    }

    pub mod float {
        macro_rules! id {
            ($value:literal) => {
                $value as f32
            };
        }
        pub(crate) use id;
    }

    pub mod string {
        pub(crate) use super::id;
    }

    pub mod character {
        pub(crate) use super::id;
    }

    pub mod byte_character {
        macro_rules! id {
            ($value:literal) => {{
                $value
            }};
        }
        pub(crate) use id;
    }

    pub mod byte_string {
        pub(crate) use super::id;
    }

    pub mod c_string {
        pub(crate) use super::id;
    }
}
