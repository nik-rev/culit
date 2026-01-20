# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/nik-rev/culit/compare/v0.6.3...HEAD

## [v0.6.3] - 2026-01-04

[v0.6.3]: https://github.com/nik-rev/culit/compare/v0.6.2...v0.6.3

## [v0.6.2] - 2026-01-04

[v0.6.2]: https://github.com/nik-rev/culit/compare/v0.6.1...v0.6.2

## [v0.6.1] - 2025-10-25

[v0.6.1]: https://github.com/nik-rev/culit/compare/v0.6.0...v0.6.1

## [v0.6.0] - 2025-10-22

[v0.6.0]: https://github.com/nik-rev/culit/compare/v0.5.2...v0.6.0

### Changed

The default invocation of `#[culit]` is identical to `#[culit(crate::custom_literal)]`,
so it is now possible to override where we look for custom literals. For example,
`#[culit(foo::bar)]` looks for literals in the `foo::bar` module.

**Migration guide:** Replace usages of `#[culit(local)]` with `#[culit(custom_literal)]`

## [v0.5.2] - 2025-10-13

[v0.5.2]: https://github.com/nik-rev/culit/compare/v0.5.1...v0.5.2

## [v0.5.1] - 2025-10-13

[v0.5.1]: https://github.com/nik-rev/culit/compare/v0.5.0...v0.5.1

## [v0.5.0] - 2025-10-13

[v0.5.0]: https://github.com/nik-rev/culit/compare/v0.4.3...v0.5.0

### Added

You can now use `#[culit(local)]` which will expand to literals at `custom_literal` (must be in scope),
rather than `crate::custom_literal`

```rust
#[culit(local)]
fn kilometer() {
    assert_eq!(10.4km, Kilometer(10.4))
    // expands to: custom_literal::km!(10.4)
}

#[culit]
fn kilomile() {
    assert_eq!(10.4km, Kilomile(10.4))
    // expands to: crate::custom_literal::km!(10.4)
}
```

## [v0.4.3] - 2025-09-22

[v0.4.3]: https://github.com/nik-rev/culit/compare/v0.4.2...v0.4.3

## [v0.4.2] - 2025-09-22

[v0.4.2]: https://github.com/nik-rev/culit/compare/v0.4.1...v0.4.2

## [v0.4.1] - 2025-09-22

[v0.4.1]: https://github.com/nik-rev/culit/compare/v0.4.0...v0.4.1

## [v0.4.0] - 2025-09-22

[v0.4.0]: https://github.com/nik-rev/culit/compare/v0.3.3...v0.4.0

### Changed

Renamed `decimal` to `float` to avoid confusion.

Even though it is not 100% correct (since floats refer to the IEEE formats, but our floats can have an arbitrary exponent,
precision and size), it'd technically be more correct to call them "real"s.
But Rust does not make this distinction, so in order to raise the minimum possible confusion let's just call them `float`s.

## [v0.3.3] - 2025-09-22

[v0.3.3]: https://github.com/nik-rev/culit/compare/v0.3.2...v0.3.3

## [v0.3.2] - 2025-09-22

[v0.3.2]: https://github.com/nik-rev/culit/compare/v0.3.1...v0.3.2

## [v0.3.1] - 2025-09-22

[v0.3.1]: https://github.com/nik-rev/culit/compare/v0.3.0...v0.3.1

## [v0.3.0] - 2025-09-22

[v0.3.0]: https://github.com/nik-rev/culit/compare/v0.2.3...v0.3.0

### Changed

- Integers and decimals are passed as-is, without splitting them up. e.g:
  - `100.003e7km` expands to `crate::custom_literal::decimal::km!(100.003e7)`
  - `100km` expands to `crate::custom_literal::integer::km!(100)`
- Byte character is also passed as-is instead of the number, e.g.: `b'a'ascii` expands to `crate::custom_literal::byte_character::ascii!(b'a')`
- MSRV is now 1.79

## [v0.2.3] - 2025-09-22

[v0.2.3]: https://github.com/nik-rev/culit/compare/v0.2.2...v0.2.3

## [v0.2.2] - 2025-09-22

[v0.2.2]: https://github.com/nik-rev/culit/compare/v0.2.1...v0.2.2

## [v0.2.1] - 2025-09-22

[v0.2.1]: https://github.com/nik-rev/culit/compare/v0.2.0...v0.2.1

## [v0.2.0] - 2025-09-22

[v0.2.0]: https://github.com/nik-rev/culit/compare/v0.1.9...v0.2.0

### Changed

Significant improvements to usability, specifically in defining custom integer and decimal literals.

#### Renamed the modules that we expect at `crate::custom_literal` to be more descriptive

|old|new|
|---|---|
|`int`|`integer`|
|`float`|`decimal`|
|`char`|`character`|
|`byte_char`|`byte_character`|
|`byte_str`|`byte_string`|
|`str`|`string`|
|`c_str`|`c_string`|

We renamed from Float because float is too-specific to the format but decimal is a more general name for what we actually give you

#### The signature of custom integer literal has changed.

- No more base, we handle that for you.
- No more strings, you get the actual number.

```rs
10km // crate::custom_literal::integer::km!(10)
0b10km // crate::custom_literal::integer::km!(3)
-10km // -crate::custom_literal::integer::km!(10)
```

Limitation: The absolute value of the custom literal may not exceed `340_282_366_920_938_463_463_374_607_431_768_211_455`

#### The signature of custom decimal literal has changed

- No more strings. Fractional, integral and the exponent parts are now numbers.
Exponent also contains the `-` sign

```rs
10.0km // crate::custom_literal::decimal::km!(10 0 1)
10e7km // crate::custom_literal::decimal::km!(10 0 7)
10e-7km // crate::custom_literal::decimal::km!(10 0 -7)
-10e-7km // -crate::custom_literal::decimal::km!(10 0 -7)
-10.4e7km // -crate::custom_literal::decimal::km!(10 4 7)
```

Limitation: Each of these may not exceed `340_282_366_920_938_463_463_374_607_431_768_211_455`:

- Integral part (part before the decimal point)
- Fractional part (part after the decimal point, before the exponent)
- Exponent

## [v0.1.9] - 2025-09-21

[v0.1.9]: https://github.com/nik-rev/culit/compare/v0.1.8...v0.1.9

## [v0.1.8] - 2025-09-21

[v0.1.8]: https://github.com/nik-rev/culit/compare/v0.1.7...v0.1.8

## [v0.1.7] - 2025-09-21

[v0.1.7]: https://github.com/nik-rev/culit/compare/v0.1.6...v0.1.7

## [v0.1.6] - 2025-09-21

[v0.1.6]: https://github.com/nik-rev/culit/compare/v0.1.5...v0.1.6

## [v0.1.5] - 2025-09-21

[v0.1.5]: https://github.com/nik-rev/culit/compare/v0.1.4...v0.1.5

## [v0.1.4] - 2025-09-21

[v0.1.4]: https://github.com/nik-rev/culit/compare/v0.1.3...v0.1.4

## [v0.1.3] - 2025-09-21

[v0.1.3]: https://github.com/nik-rev/culit/compare/v0.1.2...v0.1.3

## [v0.1.2] - 2025-09-21

[v0.1.2]: https://github.com/nik-rev/culit/compare/v0.1.1...v0.1.2

## [v0.1.1] - 2025-09-21

[v0.1.1]: https://github.com/nik-rev/culit/compare/v0.1.0...v0.1.1
