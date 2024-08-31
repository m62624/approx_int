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

**In total, this representation uses 24 bits**. The approximate number will typically be smaller than the original, 
although this may not always be the case (for instance, a value of –88 may return a value of –88). 
In most instances (depending on the input bit length, as larger input data such as 128 bits reduced to 24 bits 
inevitably leads to some duplication), the approximate value will be less, but the reverse may occur, 
especially with negative values in the range –1 to –65. Despite some loss in precision, this method is useful in situations 
where exact values are not essential.

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

---
# License
 [MIT License](https://github.com/m62624/approx_int/blob/main/LICENSE)