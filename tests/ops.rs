use std::u32;

use approx_int::SmallValue;

#[test]
fn add_t_0() {
    assert_eq!((SmallValue::new(128) + SmallValue::new(128)).min_bits(), 8);
}

#[test]
fn add_t_1() {
    assert_eq!((SmallValue::new(128) + SmallValue::new(64)).min_bits(), 8);
}

#[should_panic(expected = "overflow")]
#[test]
fn add_t_2() {
    let _c = SmallValue::new(u32::MAX) + SmallValue::new(u32::MAX);
}

#[test]
fn add_t_3() {
    assert!(SmallValue::new(u32::MAX)
        .checked_add(SmallValue::new(u32::MAX))
        .is_none());
}

#[test]
fn sub_t_0() {
    assert_eq!((SmallValue::new(128) - SmallValue::new(128)).min_bits(), 1);
}

#[test]
fn sub_t_1() {
    assert_eq!((SmallValue::new(128) - SmallValue::new(64)).min_bits(), 6);
}

#[test]
fn sub_t_2() {
    assert!(SmallValue::new(u32::MIN)
        .checked_sub(SmallValue::new(u32::MAX))
        .is_none());
}

#[test]
fn div_t_0() {
    assert_eq!((SmallValue::new(128) / SmallValue::new(128)).min_bits(), 1);
}

#[test]
fn div_t_1() {
    assert_eq!((SmallValue::new(128) / SmallValue::new(64)).min_bits(), 2);
}

#[test]
fn div_t_2() {
    assert!(SmallValue::new(u32::MAX)
        .checked_div(SmallValue::new(u32::MIN))
        .is_none());
}

#[test]
fn mul_t_0() {
    assert_eq!((SmallValue::new(128) * SmallValue::new(128)).min_bits(), 14);
}

#[test]
fn mul_t_1() {
    assert_eq!(
        (SmallValue::new(128u32) * SmallValue::new(128u32)).min_bits(),
        14
    );
}

#[test]
fn mul_t_2() {
    assert!(SmallValue::new(u32::MAX)
        .checked_mul(SmallValue::new(u32::MAX))
        .is_none());
}

#[test]
fn rem_t_0() {
    assert_eq!((SmallValue::new(128) % SmallValue::new(128)).min_bits(), 1);
}

#[test]
fn rem_t_1() {
    assert_eq!((SmallValue::new(128) % SmallValue::new(64)).min_bits(), 1);
}

#[test]
fn rem_t_2() {
    assert!(SmallValue::new(u32::MAX)
        .checked_rem(SmallValue::new(u32::MIN))
        .is_none());
}
