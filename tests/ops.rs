use std::u32;

use approx_int::SmallValue;

#[test]
fn add_t_0() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(128);
    let c = a + b;
    assert_eq!(c.min_bits(), 8)
}

#[test]
fn add_t_1() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(64);
    let c = a + b;
    assert_eq!(c.min_bits(), 8);
}

#[should_panic(expected = "overflow")]
#[test]
fn add_t_2() {
    let a = SmallValue::new(u32::MAX);
    let b = SmallValue::new(u32::MAX);
    let _c = a + b;
}

#[test]
fn sub_t_0() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(128);
    let c = a - b;
    assert_eq!(c.min_bits(), 1)
}

#[test]
fn sub_t_1() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(64);
    let c = a - b;
    assert_eq!(c.min_bits(), 6);
}

#[test]
fn div_t_0() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(128);
    let c = a / b;
    assert_eq!(c.min_bits(), 1)
}

#[test]
fn div_t_1() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(64);
    let c = a / b;
    // percent.saturating_add(1)
    assert_eq!(c.min_bits(), 2);
}

#[test]
fn mul_t_0() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(128);
    let c = a * b;
    assert_eq!(c.min_bits(), 14)
}

#[test]
fn mul_t_1() {
    let a = SmallValue::new(128u32);
    let b = SmallValue::new(128u32);
    let c = a * b;
    assert_eq!(c.min_bits(), 14)
}

#[test]
fn rem_t_0() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(128);
    let c = a % b;
    assert_eq!(c.min_bits(), 1)
}

#[test]
fn rem_t_1() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(64);
    let c = a % b;
    assert_eq!(c.min_bits(), 1);
}
