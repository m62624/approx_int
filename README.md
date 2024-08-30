# approx_int
[![Static Badge](https://img.shields.io/badge/changelog---?style=for-the-badge&labelColor=blue&color=blue)](https://github.com/m62624/approx_int/blob/main/CHANGELOG.md)
[![Codecov](https://img.shields.io/codecov/c/github/m62624/approx_int?style=for-the-badge
)](https://app.codecov.io/gh/m62624/approx_int)


## About

This library provides a utility for approximating large numbers by
calculating the number of bits needed to store a number (similar to determining the exponent in mathematics).
The algorithm determines the maximum value that can be represented using
this bit length, and then finds the nearest percentage value that can
approximately match the original number. This approximation reduces the
size of the number, while retaining enough information for practical use.

The compact representation of numbers using a tuple `(u8, u8, bool)`:

- `u8` for the number of bits required to store the value.
- `u8` for the percentage that describes the degree of approximation.
- `bool` for storing the sign of the number (true if negative).

**In total, this representation uses 24 bits**. The approximate number will always be lower than the original one, with the exception of only the number 0 .While there is some loss of precision, this approach is useful in scenarios where exact values are not critical.

```rust
    let big_value: u128 = 8838183818381831838138182391233;
    let small_value: (u8, u8, bool) = SmallValue::new(big_value).into();

    println!("The raw value: {:?}", small_value);
    println!("{:<20} {}", "The original value:", big_value);

    let small_value: SmallValue<u128> = small_value.into();

    println!(
        "{:<20} {}",
        "The approximate value:",
        small_value.approximate()
    );
```

```
---- stdout ----
The raw value: (103, 87, false)
The original value:    8838183818381831838138182391233
The approximate value: 8822848225945509419002221297664 

```

---
# License
 [MIT License](https://github.com/m62624/approx_int/blob/main/LICENSE)