use approx_int::SmallValue;

#[test]
fn bound_t_0() {
    let value = u64::MAX;
    let small_value: SmallValue<u64> = value.into();
    let (min, max) = small_value.bounds();
    assert_ne!(max, 0);
    assert!(min < value);
}

#[test]
fn bound_t_1() {
    let value = u64::MAX - 10_000;
    let small_value: SmallValue<u64> = value.into();
    let (min, max) = small_value.bounds();
    assert!(min < value && value < max);
}

#[test]
fn bound_t_2() {
    let value = 0;
    let small_value: SmallValue<u64> = value.into();
    let (min, max) = small_value.bounds();
    assert!(min == 0 && max == 0);
}
#[test]
fn bound_t_3() {
    let value = i64::MAX;
    let small_value: SmallValue<i64> = value.into();
    let (min, max) = small_value.bounds();
    assert_ne!(max, 0);
    assert!(min < value);
}

#[test]
fn bound_t_4() {
    let value = i64::MAX - 10_000;
    let small_value: SmallValue<i64> = value.into();
    let (min, max) = small_value.bounds();
    assert!(min < value && value < max);
}

#[test]
fn bound_t_5() {
    let value = i64::MIN;
    let small_value: SmallValue<i64> = value.into();
    let (min, max) = small_value.bounds();
    assert_ne!(min, 0);
    assert!(value < max);
}

#[test]
fn bound_t_6() {
    let value = -10_000;
    let small_value: SmallValue<i64> = value.into();
    let (min, max) = small_value.bounds();
    assert!(min < value && value < max);
}

#[test]
fn bound_t_7() {
    let value = i64::MIN + 10_000;
    let small_value: SmallValue<i64> = value.into();
    let (min, max) = small_value.bounds();
    assert!(min < value && value < max);
}

#[test]
fn bound_t_8() {
    let value = u64::MIN + 10_000;
    let small_value: SmallValue<u64> = value.into();
    let (min, max) = small_value.bounds();
    assert!(min < value && value < max);
}
