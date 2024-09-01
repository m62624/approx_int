# approx_int
[![Crates.io](https://img.shields.io/crates/v/approx_int?style=for-the-badge
)](https://crates.io/crates/approx_int)
[![Docs.rs](https://img.shields.io/docsrs/approx_int?style=for-the-badge
)](https://docs.rs/approx_int/latest/approx_int)
[![Static Badge](https://img.shields.io/badge/changelog---?style=for-the-badge&labelColor=blue&color=blue)](https://github.com/m62624/approx_int/blob/main/CHANGELOG.md)
[![Codecov](https://img.shields.io/codecov/c/github/m62624/approx_int?style=for-the-badge
)](https://app.codecov.io/gh/m62624/approx_int)


## About

This library provides a utility for approximating large numbers by
calculating the number of bits needed to store a number.
The algorithm determines the maximum value that can be represented using
this bit length, and then finds the nearest percentage value that can
approximately match the original number. This approximation reduces the
size of the number, while retaining enough information for practical use.

The compact representation of numbers using a tuple `(u8, u8, bool)`:

- `u8` for the number of bits required to store the value.
- `u8` for the percentage that describes the degree of approximation.
- `bool` for storing the sign of the number (true if negative).

 **In total, this representation uses 24 bits**. The approximate number will generally be 
 smaller than the original, although there are exceptions, especially when working with negative values. 
 For positive numbers, the approximation usually results in a slightly smaller value, 
 but with negative numbers, the approximation could be either smaller or 
 larger than the original (for example, a value of –88 might approximate to –88 or a slightly different value).

```rust
    let big_value: u128 = 8838183818381831838138182391233;
    let small_value = SmallValue::new(big_value);
    let tuple: (u8, u8, bool) = small_value.into();

    println!("The raw value: {:?}", tuple);
    println!("{:<25} {}", "The original value:", big_value);
    println!(
        "{:<25} {}",
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

### Approximation Method
This method relies on several key concepts:

- Determining the minimum Number of bits required to store a number. You calculate the minimum number of bits necessary to represent a number, which is then used to determine the maximum value that can be represented with that number of bits (`2^(n-1)`).

- Selecting the optimal percentage for approximation. A percentage is used to determine how closely the approximated value should match the original number. This percentage is chosen to minimize the difference between the original and the approximation.

Approximation Formula
The formula for calculating the approximated value is as follows:

```
approximate_value = (bit_pow(min_bits) * percent) / 100
```

Where:

- `bit_pow(min_bits)` — is the maximum value that can be represented by the given number of bits (min_bits).
- `percent` — is the percentage value chosen to minimize the approximation error.

---
# License
 [MIT License](https://github.com/m62624/approx_int/blob/main/LICENSE)