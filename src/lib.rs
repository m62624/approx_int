//! This library approximates large numbers by calculating the number of bits needed to store a number.
//! The algorithm determines the maximum value that can be represented using this bit length,
//! and then finds the nearest percentage value that can approximately match the original number.
//! This approximation reduces the size of the number while retaining enough information for practical use.
//!
//! The compact representation of numbers uses a tuple `(u8, u8, bool)`:
//!
//! - `u8` for the number of bits required to store the value.
//! - `u8` for the percentage that describes the degree of approximation.
//! - `bool` for storing the sign of the number (true if negative).
//!
//! **In total, this representation uses 24 bits**, but you can omit the `bool` if you are sure the number is positive,
//! then **only 16 bits will be needed**. The approximate number will generally be smaller than the original,
//! although there are exceptions, especially when working with negative values.
//! For positive numbers, the approximation usually results in a slightly smaller value,
//! but with negative numbers, the approximation could be either smaller or larger than the original.

use num_traits::{CheckedRem, CheckedShl, PrimInt};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
/// The structure stores the resulting number
/// in a compressed format from which an approximated number can be obtained
pub struct SmallValue<T: SpecialBytes> {
    min_bits: u8,
    percent: u8,
    flag: bool,
    _phantom: PhantomData<T>,
}

/// The trait is used to determine the number of bits required to store a number
pub trait SpecialBytes: PrimInt + Default + CheckedShl + CheckedRem {
    /// Returns the number of bits required to store a number
    fn bits() -> u8;
}

macro_rules! impl_default_bits {
    ($($t:ty => $b:expr),*) => {
        $(
            impl SpecialBytes for $t {
                fn bits() -> u8 {
                    $b
                }
            }
        )*
    };
}

impl_default_bits! {
    u32 => 32,
    u64 => 64,
    u128 => 128,
    i32 => 32,
    i64 => 64,
    i128 => 128
}

impl<T: SpecialBytes> SmallValue<T> {
    // Calculate the number of bits required to represent a number.
    fn bit_size(number: T) -> u8 {
        (number == T::zero()).then_some(1).unwrap_or_else(|| {
            T::bits().saturating_sub(if number < T::zero() {
                (!number).leading_zeros()
            } else {
                number.leading_zeros()
            } as u8)
        })
    }

    // Calculate the maximum value that can be represented using a given number of bits.
    fn bit_pow(power: u8) -> T {
        if power >= T::bits() {
            T::max_value()
        } else {
            match T::one().checked_shl(power.into()) {
                Some(shifted) => shifted.checked_sub(&T::one()).unwrap_or(T::max_value()),
                None => T::max_value(),
            }
        }
    }

    // Calculate the approximate value based on a percentage.
    fn calculate_part_from_percentage(percentage: u8, total: T) -> T {
        let hundred = T::from(100u8).unwrap_or_default();
        let percentage = T::from(percentage).unwrap_or_default();

        if hundred > total {
            if let Some(product) = total.checked_mul(&percentage) {
                if let Some(result) = product.checked_div(&hundred) {
                    return result;
                }
            }
        }

        total
            .checked_div(&hundred)
            .and_then(|part| part.checked_mul(&percentage))
            .unwrap_or_else(T::zero)
    }

    fn calculate_error_rate(original: T, approximate: T) -> T {
        let diff = original
            .checked_sub(&approximate)
            .and_then(|sub| sub.to_f64())
            .and_then(|diff| original.to_f64().map(|original| diff / original))
            .map(|div| div * 100.0)
            .unwrap_or_default();

        if let Some(percent) = T::from(diff) {
            percent
        } else {
            T::zero()
        }
    }
}

impl<T: SpecialBytes> SmallValue<T> {
    /// Create a new instance of SmallValue.
    ///
    /// ---
    /// You can use `Into` to convert a number to a `SmallValue`.
    /// ### Example
    /// ```rust
    /// let value: SmallValue<i32> = 123.into();
    /// ```
    ///
    /// ---
    /// after creation, the number can be represented in a smaller
    /// representation `(min_bits: u8, percent:u8, flag: bool)` (use `into`).
    /// ### Example
    /// ```rust
    /// let tuple: (u8, u8, bool) = small_value.into();
    /// // and the reverse operation
    /// let tuple = (8, 50, false);
    /// let small_value: SmallValue<u32> = tuple.into();
    /// ```
    pub fn new(number: T) -> Self {
        let min_bits = Self::bit_size(number);
        let mut percent = 99;

        let (abs_number, flag) = if number < T::zero() {
            if T::min_value() == number {
                return Self {
                    min_bits,
                    percent,
                    flag: true,
                    _phantom: PhantomData,
                };
            }
            (T::zero() - number, true)
        } else {
            (number, false)
        };

        while percent > 1 {
            let approx = Self::calculate_part_from_percentage(percent, Self::bit_pow(min_bits));
            if abs_number > approx {
                return Self {
                    min_bits,
                    percent: if flag {
                        percent
                            + Self::calculate_error_rate(
                                abs_number,
                                SmallValue::from((min_bits, percent, false)).approximate(),
                            )
                            .to_u8()
                            .unwrap_or_default()
                    } else {
                        percent
                    },
                    flag,
                    _phantom: PhantomData,
                };
            }
            percent -= 1;
        }

        Self {
            min_bits,
            percent: 1,
            flag,
            _phantom: PhantomData,
        }
    }

    /// Returns the approximate value of the number.
    /// The approximate number will usually be less than the original one,
    /// although there are exceptions, especially when working with negative values or with small numbers (less than 100).
    ///
    /// ### Example
    /// ```rust
    /// let big_value: u128 = 8838183818381831838138182391233;
    /// let small_value = SmallValue::new(big_value);
    /// let approx = small_value.approximate();
    /// // ---- stdout ----
    /// // The raw value: (103, 87, false)
    /// // The original value:    8838183818381831838138182391233
    /// // The approximate value: 8822848225945509419002221297664
    /// ```
    pub fn approximate(&self) -> T {
        let abs_value = Self::calculate_part_from_percentage(
            if self.flag {
                self.percent.saturating_add(1)
            } else {
                self.percent
            },
            Self::bit_pow(self.min_bits),
        );

        if self.flag {
            T::zero() - abs_value
        } else {
            abs_value
        }
    }

    /// Returns a tuple with the minimum and maximum values for this object.
    ///
    /// The minimum value (`min`) is calculated using the `approximate` method.
    /// The maximum value (`max`) is calculated by creating a new object with an increased percentage value and calling the `approximate` method.
    ///
    /// It is generally guaranteed that the original number is not less than `min`, but also not greater than `max`.
    /// However, there may be exceptions for negative numbers. Additionally, if the number is close to the maximum value of the chosen type (`T::MAX`),
    /// the `max` value might be lower than the original number.
    /// For example, if the type `T` is `u32`, and the number is close to `u32::MAX`, then the `max` boundary will likely be lower than the original number (the same thing can happen if `min` is close to `u32::MIN`).
    ///
    pub fn bounds(&self) -> (T, T) {
        let min = self.approximate();

        (
            min,
            Self::from((
                self.min_bits,
                if self.flag {
                    self.percent.saturating_sub(2)
                } else {
                    self.percent.saturating_add(1)
                },
                self.flag,
            ))
            .approximate(),
        )
    }

    /// Returns the minimum number of bits required to represent the number.
    #[cfg(not(tarpaulin_include))]
    pub fn min_bits(&self) -> u8 {
        self.min_bits
    }

    /// This percentage is derived from the maximum value of the bit representation
    #[cfg(not(tarpaulin_include))]
    pub fn percent(&self) -> u8 {
        self.percent
    }

    /// Returns the flag that indicates whether the number is negative.
    #[cfg(not(tarpaulin_include))]
    pub fn flag(&self) -> bool {
        self.flag
    }
}

#[cfg(not(tarpaulin_include))]
impl<T: SpecialBytes> std::fmt::Display for SmallValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Exponent: {}, Percentage: {}",
            self.min_bits, self.percent
        )
    }
}

#[cfg(not(tarpaulin_include))]
impl<T: SpecialBytes> Default for SmallValue<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: SpecialBytes> PartialOrd for SmallValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: SpecialBytes> Ord for SmallValue<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.approximate().cmp(&other.approximate())
    }
}

impl<T: SpecialBytes> From<(u8, u8, bool)> for SmallValue<T> {
    fn from((min_bits, percent, flag): (u8, u8, bool)) -> Self {
        Self {
            min_bits,
            percent,
            flag,
            _phantom: PhantomData,
        }
    }
}

impl<T: SpecialBytes> From<(u8, u8)> for SmallValue<T> {
    fn from((min_bits, percent): (u8, u8)) -> Self {
        Self {
            min_bits,
            percent,
            flag: false,
            _phantom: PhantomData,
        }
    }
}

impl<T: SpecialBytes> From<SmallValue<T>> for (u8, u8, bool) {
    fn from(value: SmallValue<T>) -> Self {
        (value.min_bits, value.percent, value.flag)
    }
}

impl<T: SpecialBytes> From<SmallValue<T>> for (u8, u8) {
    fn from(value: SmallValue<T>) -> Self {
        (value.min_bits, value.percent)
    }
}

impl<T: SpecialBytes> From<T> for SmallValue<T> {
    fn from(number: T) -> Self {
        Self::new(number)
    }
}

impl<T: SpecialBytes> Add for SmallValue<T> {
    type Output = SmallValue<T>;

    fn add(self, rhs: Self) -> Self::Output {
        SmallValue::new(self.approximate() + rhs.approximate())
    }
}

impl<T: SpecialBytes> Sub for SmallValue<T> {
    type Output = SmallValue<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        SmallValue::new(self.approximate() - rhs.approximate())
    }
}

impl<T: SpecialBytes> Mul for SmallValue<T> {
    type Output = SmallValue<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        SmallValue::new(self.approximate() * rhs.approximate())
    }
}

impl<T: SpecialBytes> Div for SmallValue<T> {
    type Output = SmallValue<T>;

    fn div(self, rhs: Self) -> Self::Output {
        SmallValue::new(self.approximate() / rhs.approximate())
    }
}

impl<T: SpecialBytes> Rem for SmallValue<T> {
    type Output = SmallValue<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        SmallValue::new(self.approximate() % rhs.approximate())
    }
}

// wrapping, checked:  add,sub, mul, div, rem
impl<T: SpecialBytes> SmallValue<T> {
    /// Checked addition. Returns `None` if overflow occurred.
    pub fn checked_add(&self, rhs: Self) -> Option<Self> {
        self.approximate()
            .checked_add(&rhs.approximate())
            .map(Self::new)
    }

    /// Checked subtraction. Returns `None` if overflow occurred.
    pub fn checked_sub(&self, rhs: Self) -> Option<Self> {
        self.approximate()
            .checked_sub(&rhs.approximate())
            .map(Self::new)
    }

    /// Checked multiplication. Returns `None` if overflow occurred.
    pub fn checked_mul(&self, rhs: Self) -> Option<Self> {
        self.approximate()
            .checked_mul(&rhs.approximate())
            .map(Self::new)
    }

    /// Checked division. Returns `None` if the divisor is zero or if the result overflows.
    pub fn checked_div(&self, rhs: Self) -> Option<Self> {
        self.approximate()
            .checked_div(&rhs.approximate())
            .map(Self::new)
    }

    /// Checked remainder. Returns `None` if the divisor is zero or if the result overflows.
    pub fn checked_rem(&self, rhs: Self) -> Option<Self> {
        self.approximate()
            .checked_rem(&rhs.approximate())
            .map(Self::new)
    }
}
