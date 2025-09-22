# `culit` - Custom Literals in Rust

<!-- cargo-rdme start -->

[![crates.io](https://img.shields.io/crates/v/culit?style=flat-square&logo=rust)](https://crates.io/crates/culit)
[![docs.rs](https://img.shields.io/badge/docs.rs-culit-blue?style=flat-square&logo=docs.rs)](https://docs.rs/culit)
![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)
![msrv](https://img.shields.io/badge/msrv-1.58-blue?style=flat-square&logo=rust)
[![github](https://img.shields.io/github/stars/nik-rev/culit)](https://github.com/nik-rev/culit)

You probably know that numbers in Rust can be suffixed to specify their type, e.g. `100i32`.
But did you know that syntactically *any* literal can have a suffix? **And did you know that the suffix can be whatever you want**?

This crate provides an attribute macro `#[culit]` for "Custom Literals". When applied to any statement, it enables using custom literals in that statement.

```toml
[dependencies]
culit = "0.3"
```

Note: `culit` does not have any dependencies such as `syn` or `quote`, and it is a simple mapping `SourceCode -> SourceCode`, so compile-speeds will be very fast.

## Example

A [`NonZeroUsize`](std::num::NonZeroUsize) literal that fails to compile if it is `0`: `100nzusize`

```rust
use culit::culit;
use std::num::NonZeroUsize;

#[culit]
fn main() {
    assert_eq!(100nzusize, NonZeroUsize::new(100).unwrap());
    // COMPILE ERROR!
    // let illegal = 0nzusize;
}

mod custom_literal {
    pub mod integer {
        macro_rules! nzusize {
            // handle `0` specially
            (0) => {
                compile_error!("`0` is not a valid `NonZeroUsize`")
            };
            ($value:literal) => {
                const { NonZeroUsize::new($value).unwrap() }
            };
        }
        pub(crate) use nzusize;
    }
}
```

## IDE Support

Hovering over the custom literals shows documentation for the macro that generates them. You can also do "goto definition". It's quite nice!

![IDE Support](https://raw.githubusercontent.com/nik-rev/culit/71f4a2b32eb87b955d0c953bd3e90e80bd6a938d/ide_support.png)

## More Examples


Python-like f-strings: `"hello {name}"f`

```rust
use culit::culit;
use std::time::Duration;

#[culit]
fn main() {
    let name = "bob";
    let age = 23;

    assert_eq!(
        "hi, my name is {name} and I am {age} years old"f,
        format!("hi, my name is {name} and I am {age} years old")
    );
}

mod custom_literal {
    pub mod string {
        macro_rules! f {
            ($value:literal) => {
                format!($value)
            };
        }
        pub(crate) use f;
    }
}
```

[`Duration`](std::time::Duration) literals: `100m`, `2h`...

```rust
use culit::culit;
use std::time::Duration;

#[culit]
fn main() {
    assert_eq!(
        100d + 11h + 8m + 7s,
        Duration::from_secs(100 * 60 * 60 * 24)
        + Duration::from_secs(11 * 60 * 60)
        + Duration::from_secs(8 * 60)
        + Duration::from_secs(7)
    );
}

mod custom_literal {
    pub mod integer {
        // day
        macro_rules! d {
            ($value:literal) => {
                Duration::from_secs(60 * 60 * 24 * $value)
            };
        }
        pub(crate) use d;

        // hour
        macro_rules! h {
            ($value:literal) => {
                Duration::from_secs(60 * 60 * $value)
            };
        }
        pub(crate) use h;

        // minute
        macro_rules! m {
            ($value:literal) => {
                Duration::from_secs(60 * $value)
            };
        }
        pub(crate) use m;

        // second
        macro_rules! s {
            ($value:literal) => {
                Duration::from_secs($value)
            };
        }
        pub(crate) use s;
    }
}
```

The possibilities are *endless!*

## Details

`#[culit]` replaces every literal that has a custom suffix with a call to the macro
at `crate::custom_literal::<type>::<suffix>!($value)`, where `$value` is the literal with the suffix stripped:

|literal|expansion|
|---|---|
| `100km` | `crate::custom_literal::integer::km!(100)` |
| `70.008e7feet` | `crate::custom_literal::decimal::feet!(70.008e7)` |
| `"foo"bar` | `crate::custom_literal::string::bar!("foo")` |
| `'a'ascii` | `crate::custom_literal::character::ascii!('a')` |
| `b"foo"bar` | `crate::custom_literal::byte_string::bar!(b"foo")` |
| `b'a'ascii` | `crate::custom_literal::byte_character::ascii!(b'a')` |
| `c"foo"bar` | `crate::custom_literal::c_string::bar!(c"foo")` |

Notes:

- Built-in suffixes like `usize` and `f32` do **not** expand, so you cannot overwrite them.
- Escapes are fully processed, so there's no `raw_byte_str`. `rb#"f\oo"#` just becomes `b"f\\oo"`

### Skeleton

Here's a skeleton for the `custom_literal` module which must exist at `crate::custom_literal`.
This module adds a new literal for every type of literal:

```rust
mod custom_literal {
    pub mod integer {
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod decimal {
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod string {
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod character {
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod byte_character {
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod byte_string {
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod c_string {
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }
}
```

## Nightly

You need to use `#[culit]` attribute everywhere you want to use these literals. On nightly, you can apply it on the module:

```rust
#![feature(custom_inner_attributes)]
#![feature(proc_macro_hygiene)]
#![culit::culit]
```

While this *works*, I wouldn't recommend it - currently rust-analyzer is unable to properly work with custom inner attributes
that modify the whole crate. For example, if you write `0nzusize` which produces a compiler error, the span of the error will point to
the macro `crate::custom_literal::int::nzusize` but *not* the actual `0nzusize`, which makes it very hard to debug these

<!-- cargo-rdme end -->
