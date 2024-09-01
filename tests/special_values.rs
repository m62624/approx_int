use approx_int::{SmallValue, SpecialBytes};

fn calculate_error_rate<T: SpecialBytes>(original: T, approximate: T) -> Option<f64> {
    if original.is_zero() {
        None
    } else {
        Some(((original - approximate).to_f64()? / original.to_f64()?) * 100.0)
    }
}

#[test]
fn special_value_t_0() {
    let original = 314265047194861710702393853117681977664u128;
    let approx = SmallValue::new(original).approximate();
    let error_rate = calculate_error_rate(original, approx).unwrap();
    assert!(error_rate < 0.9);
}

#[test]
fn special_value_t_1() {
    let original = 283133942661148271300918452068693765763u128;
    let approx = SmallValue::new(original).approximate();
    let error_rate = calculate_error_rate(original, approx).unwrap();
    assert!(error_rate < 0.9);
}

#[test]
fn special_value_t_2() {
    let original = 188221493376348417689459869480508947014u128;
    let approx = SmallValue::new(original).approximate();
    let error_rate = calculate_error_rate(original, approx).unwrap();
    assert!(error_rate < 0.9);
}

#[test]
fn special_value_t_3() {
    let original = -324324923040329432943249324903294i128;
    let approx = SmallValue::new(original).approximate();
    let error_rate = calculate_error_rate(original, approx).unwrap();
    assert!(error_rate < 1.0);
}

#[test]
fn special_value_t_4() {
    let original = -14403;
    let approx = SmallValue::new(original).approximate();
    let error_rate = calculate_error_rate(original, approx).unwrap();
    assert!(error_rate < 1.0);
}

#[test]
fn special_value_t_5() {
    for i in -1000..-5 {
        let original = i;
        let approx = SmallValue::new(original).approximate();
        let error_rate = calculate_error_rate(original, approx).unwrap();
        assert!(error_rate < 10.0);
    }
}

#[test]
fn special_value_t_6() {
    let original = i32::MIN;
    assert!(calculate_error_rate(original, SmallValue::new(original).approximate()).unwrap() < 2.0);
}

#[test]
fn special_value_t_7() {
    let original = i32::MAX;
    assert!(calculate_error_rate(original, SmallValue::new(original).approximate()).unwrap() < 2.0);
}

#[test]
fn special_value_t_8() {
    let original = i64::MIN;
    assert!(calculate_error_rate(original, SmallValue::new(original).approximate()).unwrap() < 2.0);
}

#[test]
fn special_value_t_9() {
    let original = i64::MAX;
    assert!(calculate_error_rate(original, SmallValue::new(original).approximate()).unwrap() < 2.0);
}

#[test]
fn special_value_t_10() {
    let original = i128::MIN;
    assert!(calculate_error_rate(original, SmallValue::new(original).approximate()).unwrap() < 2.0);
}

#[test]
fn special_value_t_11() {
    let original = i128::MAX;
    assert!(calculate_error_rate(original, SmallValue::new(original).approximate()).unwrap() < 2.0);
}

#[test]
fn special_value_t_12() {
    let original = u32::MAX;
    assert!(calculate_error_rate(original, SmallValue::new(original).approximate()).unwrap() < 2.0);
}

#[test]
fn special_value_t_13() {
    let original = u64::MAX;
    assert!(calculate_error_rate(original, SmallValue::new(original).approximate()).unwrap() < 2.0);
}

#[test]
fn special_value_t_14() {
    let original = u128::MAX;
    assert!(calculate_error_rate(original, SmallValue::new(original).approximate()).unwrap() < 2.0);
}
