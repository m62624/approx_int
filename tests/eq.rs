use approx_int::SmallValue;

#[test]
fn eq_t_0() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(128);
    assert_eq!(a, b);
}

#[test]
fn eq_t_1() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(64);
    assert_ne!(a, b);
}

#[test]
fn eq_t_2() {
    let a = SmallValue::new(127);
    let b = SmallValue::new(128);
    assert!(a < b);
}

#[test]
fn eq_t_3() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(128);
    assert!(a <= b);
}

#[test]
fn eq_t_4() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(127);
    assert!(a > b);
}

#[test]
fn eq_t_5() {
    let a = SmallValue::new(128);
    let b = SmallValue::new(128);
    assert!(a >= b);
}

