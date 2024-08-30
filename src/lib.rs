use num_traits::PrimInt;
use std::marker::PhantomData;

pub trait DefaultBits {
    fn bits() -> u8;
}

impl DefaultBits for u32 {
    fn bits() -> u8 {
        32
    }
}

impl DefaultBits for u64 {
    fn bits() -> u8 {
        64
    }
}

impl DefaultBits for u128 {
    fn bits() -> u8 {
        128
    }
}

impl DefaultBits for i32 {
    fn bits() -> u8 {
        32
    }
}

impl DefaultBits for i64 {
    fn bits() -> u8 {
        64
    }
}

impl DefaultBits for i128 {
    fn bits() -> u8 {
        128
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(any(feature = "debug", test), derive(Debug))]
pub struct SmallValue<T: PrimInt + DefaultBits> {
    min_bits: u8,
    percent: u8,
    flag: bool,
    _phantom: PhantomData<T>,
}

impl<T: PrimInt + DefaultBits> SmallValue<T> {
    fn bit_size(number: T) -> u8 {
        if number == T::zero() {
            return 1;
        } else {
            T::bits() - number.leading_zeros() as u8
        }
    }

    fn bit_pow(power: u8) -> T {
        if power >= T::bits() {
            T::max_value()
        } else {
            let one = T::one();
            let shifted = one << power.into();
            shifted - one
        }
    }

    fn calculate_part_from_percentage(percentage: u8, total: T) -> T {
        if percentage == 100 {
            return total;
        }

        let total_f64 = match total.to_f64() {
            Some(value) => value,
            None => return T::zero(),
        };

        let result_f64 = total_f64 * (percentage as f64 / 100.0);

        match T::from(result_f64) {
            Some(value) => value,
            None => T::zero(),
        }
    }

    pub fn new(number: T) -> Self {
        let mut percent = 99;
        let min_bits = Self::bit_size(number);

        loop {
            if number
                .checked_sub(&Self::calculate_part_from_percentage(
                    percent,
                    Self::bit_pow(min_bits),
                ))
                .is_some()
            {
                return Self {
                    min_bits,
                    percent,
                    flag: number < T::zero(),
                    _phantom: PhantomData,
                };
            }
            percent -= 1;
        }
    }

    pub fn approximate_value(&self) -> T {
        Self::calculate_part_from_percentage(self.percent, Self::bit_pow(self.min_bits))
    }
}

#[test]
fn check() {
    let original_number = 32013013023031032u128;
    // let original_number = 382831829391923912392818382312u128;
    println!("x: {}", original_number);
    let small_value = SmallValue::new(original_number);
    println!("{:#?}", small_value);

    println!("y: {}", small_value.approximate_value());
}
