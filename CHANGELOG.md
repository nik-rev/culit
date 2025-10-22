# v0.6.0

The default invocation of `#[culit]` is identical to `#[culit(crate::custom_literal)]`,
so it is now possible to override where we look for custom literals. For example,
`#[culit(foo::bar)]` looks for literals in the `foo::bar` module.

# v0.5.0

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

# v0.4.0

Renamed `decimal` to `float` to avoid confusion.

Even though it is not 100% correct (since floats refer to the IEEE formats, but our floats can have an arbitrary exponent,
precision and size), it'd technically be more correct to call them "real"s.
But Rust does not make this distinction, so in order to raise the minimum possible confusion let's just call them `float`s.

# v0.3.0

- Integers and decimals are passed as-is, without splitting them up. e.g:
  - `100.003e7km` expands to `crate::custom_literal::decimal::km!(100.003e7)`
  - `100km` expands to `crate::custom_literal::integer::km!(100)`
- Byte character is also passed as-is instead of the number, e.g.: `b'a'ascii` expands to `crate::custom_literal::byte_character::ascii!(b'a')`
- MSRV is now 1.79

# v0.2.0

Includes significant improvements to usability, specifically in defining custom integer and decimal literals.

## Renamed the modules that we expect at `crate::custom_literal` to be more descriptive

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

## The signature of custom integer literal has changed.

- No more base, we handle that for you.
- No more strings, you get the actual number.

```rs
10km // crate::custom_literal::integer::km!(10)
0b10km // crate::custom_literal::integer::km!(3)
-10km // -crate::custom_literal::integer::km!(10)
```

Limitation: The absolute value of the custom literal may not exceed `340_282_366_920_938_463_463_374_607_431_768_211_455`

## The signature of custom decimal literal has changed

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

# v0.1.0

Initial release
