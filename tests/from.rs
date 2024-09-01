use approx_int::SmallValue;

#[test]
fn tuple_t_0() {
    let small_value = SmallValue::new(128);
    let tuple: (u8, u8, bool) = small_value.into();
    assert_eq!(tuple, (8, 63, false));
}

#[test]
fn tuple_t_1() {
    let small_value = SmallValue::new(64);
    let tuple: (u8, u8, bool) = small_value.into();
    assert_eq!(tuple, (7, 63, false));
}

#[test]
fn tuple_t_2() {
    let tuple = (8, 50, false);
    let small_value: SmallValue<u32> = tuple.into();
    assert_eq!(small_value.min_bits(), 8);
}

#[test]
fn tuple_t_3() {
    let value = u128::MAX;
    let small_value: SmallValue<u128> = value.into();
    assert_eq!(small_value.min_bits(), 128);
}
