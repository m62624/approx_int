use approx_int::SmallValue;

#[test]
fn new_t_0() {
    let small_value = SmallValue::new(u32::MAX);
    assert_eq!(small_value.min_bits(), 32)
}

#[test]
fn new_t_1() {
    let small_value = SmallValue::new(u32::MIN);
    assert_eq!(small_value.min_bits(), 1)
}

#[test]
fn new_t_2() {
    let small_value = SmallValue::new(u64::MAX);
    assert_eq!(small_value.min_bits(), 64)
}

#[test]
fn new_t_3() {
    let small_value = SmallValue::new(u64::MIN);
    assert_eq!(small_value.min_bits(), 1)
}

#[test]
fn new_t_4() {
    let small_value = SmallValue::new(u128::MAX);
    assert_eq!(small_value.min_bits(), 128)
}

#[test]
fn new_t_5() {
    let small_value = SmallValue::new(u128::MIN);
    assert_eq!(small_value.min_bits(), 1)
}

#[test]
fn new_t_6() {
    let small_value = SmallValue::new(i32::MAX);
    assert_eq!(small_value.min_bits(), 31)
}

#[test]
fn new_t_7() {
    let small_value = SmallValue::new(i32::MIN);
    assert_eq!(small_value.min_bits(), 31)
}

#[test]
fn new_t_8() {
    let small_value = SmallValue::new(i64::MAX);
    assert_eq!(small_value.min_bits(), 63)
}

#[test]
fn new_t_9() {
    let small_value = SmallValue::new(i64::MIN);
    assert_eq!(small_value.min_bits(), 63)
}

#[test]
fn new_t_10() {
    let small_value = SmallValue::new(i128::MAX);
    assert_eq!(small_value.min_bits(), 127)
}

#[test]
fn new_t_11() {
    let small_value = SmallValue::new(i128::MIN);
    assert_eq!(small_value.min_bits(), 127)
}

#[test]
fn new_t_12() {
    let small_value = SmallValue::new(0u32);
    assert_eq!(small_value.min_bits(), 1)
}

#[test]
fn new_t_13() {
    let small_value = SmallValue::new(0i32);
    assert_eq!(small_value.min_bits(), 1)
}

#[test]
fn new_t_14() {
    let small_value = SmallValue::new(-3200311i32);
    assert_eq!(small_value.min_bits(), 22)
}

#[test]
fn new_t_15() {
    let small_value = SmallValue::new(-4324882843248348348i128);
    assert_eq!(small_value.min_bits(), 62)
}
