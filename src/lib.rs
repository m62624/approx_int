//! This library provides a utility for approximating large numbers by
//!  calculating the number of bits needed to store a number
//! (similar to determining the exponent in mathematics).
//! The algorithm determines the maximum value that can be represented using
//! this bit length, and then finds the nearest percentage value that can
//!  approximately match the original number. This approximation reduces the
//!  size of the number, while retaining enough information for practical use.

use num_traits::{CheckedShl, PrimInt};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Rem, Sub};

pub trait SpecialBytes: PrimInt + Default + CheckedShl {
    fn bits() -> u8;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
/// Represents a small value with configurable minimum bits, percentage, and flag.
///
/// # Fields
///
/// - `min_bits`: The minimum number of bits required to represent the value.
/// - `percent`: The percentage value.
/// - `flag`:  A flag that indicates whether the value is negative.
///
pub struct SmallValue<T: SpecialBytes> {
    min_bits: u8,
    percent: u8,
    flag: bool,
    _phantom: PhantomData<T>,
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
        let total_f32 = match total.to_f32() {
            Some(value) => value,
            None => return T::zero(),
        };

        match T::from(total_f32 * (percentage as f32 / 100.0)) {
            Some(value) => value,
            None => T::zero(),
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
    /// representation `(min_bits: u8, percent:u8, flag: bool)` (use `into`)
    /// ### Example
    /// ```rust
    /// let tuple: (u8, u8, bool) = small_value.into();
    /// // and the reverse operation
    /// let tuple = (8, 50, false);
    /// let small_value: SmallValue<u32> = tuple.into();
    /// ```
    pub fn new(number: T) -> Self {
        let min_bits = Self::bit_size(number);

        let (abs_number, flag) = if number < T::zero() {
            if T::min_value() == number {
                return Self {
                    min_bits,
                    percent: 99,
                    flag: true,
                    _phantom: PhantomData,
                };
            }
            (T::zero() - number, true)
        } else {
            (number, false)
        };

        let mut percent = 99;

        while percent > 1 {
            let approx = Self::calculate_part_from_percentage(percent, Self::bit_pow(min_bits));
            if abs_number > approx {
                return Self {
                    min_bits,
                    percent: if flag {
                        percent.saturating_add(1)
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
    /// The value is calculated based on the percentage and the minimum number of bits.
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
        let abs_value =
            Self::calculate_part_from_percentage(self.percent, Self::bit_pow(self.min_bits));

        if self.flag {
            T::zero() - abs_value
        } else {
            abs_value
        }
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

    // Returns the flag that indicates whether the number is negative.
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

impl<T: SpecialBytes> From<SmallValue<T>> for (u8, u8, bool) {
    fn from(value: SmallValue<T>) -> Self {
        (value.min_bits, value.percent, value.flag)
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
        let result = self.approximate() + rhs.approximate();
        SmallValue::new(result)
    }
}

impl<T: SpecialBytes> Sub for SmallValue<T> {
    type Output = SmallValue<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = self.approximate() - rhs.approximate();
        SmallValue::new(result)
    }
}

impl<T: SpecialBytes> Mul for SmallValue<T> {
    type Output = SmallValue<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = self.approximate() * rhs.approximate();
        SmallValue::new(result)
    }
}

impl<T: SpecialBytes> Div for SmallValue<T> {
    type Output = SmallValue<T>;

    fn div(self, rhs: Self) -> Self::Output {
        let result = self.approximate() / rhs.approximate();
        SmallValue::new(result)
    }
}

impl<T: SpecialBytes> Rem for SmallValue<T> {
    type Output = SmallValue<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        let result = self.approximate() % rhs.approximate();
        SmallValue::new(result)
    }
}
