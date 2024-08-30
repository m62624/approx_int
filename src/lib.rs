use num_traits::PrimInt;
use std::marker::PhantomData;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(any(feature = "debug", test), derive(Debug))]
pub struct SmallValue<T: PrimInt + DefaultBits> {
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

impl<T: PrimInt + DefaultBits> SmallValue<T> {
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

        // Если процент меньше или равен 1, возвращаем минимальный процент
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
}
