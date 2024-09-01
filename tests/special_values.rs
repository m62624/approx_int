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
