use num_traits::PrimInt;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SmallValue<T: PrimInt + DefaultBits + Default> {
    min_bits: u8,
    percent: u8,
    flag: bool,
    _phantom: PhantomData<T>,
}

pub trait DefaultBits {
    fn bits() -> u8;
}

macro_rules! impl_default_bits {
    ($($t:ty => $b:expr),*) => {
        $(
            impl DefaultBits for $t {
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

impl<T: PrimInt + DefaultBits + Default> SmallValue<T> {
    fn bit_size(number: T) -> u8 {
        (number == T::zero()).then(|| 1).unwrap_or_else(|| {
            T::bits().saturating_sub(if number < T::zero() {
                (!number).leading_zeros()
            } else {
                number.leading_zeros()
            } as u8)
        })
    }

    fn bit_pow(power: u8) -> T {
        if power >= T::bits() {
            T::max_value()
        } else {
            (T::one() << power.into()) - T::one()
        }
    }

    fn calculate_part_from_percentage(percentage: u8, total: T) -> T {
        let total_f64 = match total.to_f64() {
            Some(value) => value,
            None => return T::zero(),
        };

        match T::from(total_f64 * (percentage as f64 / 100.0)) {
            Some(value) => value,
            None => T::zero(),
        }
    }
}

impl<T: PrimInt + DefaultBits + Default> SmallValue<T> {
    pub fn new(number: T) -> Self {
        let min_bits = Self::bit_size(number);

        let (abs_number, flag) = if number < T::zero() {
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

    pub fn approximate(&self) -> T {
        let abs_value =
            Self::calculate_part_from_percentage(self.percent, Self::bit_pow(self.min_bits));

        if self.flag {
            T::zero() - abs_value
        } else {
            abs_value
        }
    }

    pub fn min_bits(&self) -> u8 {
        self.min_bits
    }

    pub fn percent(&self) -> u8 {
        self.percent
    }

    pub fn flag(&self) -> bool {
        self.flag
    }
}

impl<T: PrimInt + DefaultBits + Default> Default for SmallValue<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: PrimInt + DefaultBits + Default> std::fmt::Display for SmallValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Exponent: {}, Percentage: {}",
            self.min_bits, self.percent
        )
    }
}

impl<T: PrimInt + DefaultBits + Default> From<T> for SmallValue<T> {
    fn from(number: T) -> Self {
        Self::new(number)
    }
}

impl<T: PrimInt + DefaultBits + Default> PartialOrd for SmallValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PrimInt + DefaultBits + Default> Ord for SmallValue<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.approximate().cmp(&other.approximate())
    }
}

impl<T: PrimInt + DefaultBits + Default> From<(u8, u8, bool)> for SmallValue<T> {
    fn from((min_bits, percent, flag): (u8, u8, bool)) -> Self {
        Self {
            min_bits,
            percent,
            flag,
            _phantom: PhantomData,
        }
    }
}
