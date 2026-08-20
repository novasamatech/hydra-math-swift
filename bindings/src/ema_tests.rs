use super::*;

// ten minutes period smoothing, as configured by the ema oracle pallet
const TEN_MINUTES_SMOOTHING: &str = "3369132345751865974884897103284833777";

fn ratio(result: &str) -> (u128, u128) {
    let mut parts = result.split(',');
    let n = parts.next().unwrap().parse().unwrap();
    let d = parts.next().unwrap().parse().unwrap();
    (n, d)
}

fn value(result: &str) -> f64 {
    let (n, d) = ratio(result);
    n as f64 / d as f64
}

fn iterated(iterations: u32) -> String {
    ema_iterated_price(
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        iterations,
        TEN_MINUTES_SMOOTHING.to_string(),
    )
}

fn smoothed(iterations: u32, smoothing: &str) -> String {
    ema_iterated_price(
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        iterations,
        smoothing.to_string(),
    )
}

fn falling(iterations: u32) -> String {
    ema_iterated_price(
        "3".to_string(),
        "4".to_string(),
        "1".to_string(),
        "2".to_string(),
        iterations,
        TEN_MINUTES_SMOOTHING.to_string(),
    )
}

// ema_iterated_price

#[test]
fn iterated_price_should_keep_prev_when_not_outdated() {
    assert_eq!(ratio(&iterated(0)), (1, 2));
}

#[test]
fn iterated_price_should_move_towards_incoming() {
    let value = value(&iterated(1));
    assert!(value > 0.5);
    assert!(value < 0.75);
}

#[test]
fn iterated_price_should_approach_incoming_with_more_iterations() {
    assert!(value(&iterated(100)) > value(&iterated(10)));
}

#[test]
fn iterated_price_should_saturate_to_incoming() {
    assert_eq!(value(&iterated(u32::MAX)), 0.75);
}

#[test]
fn iterated_price_should_use_the_smoothing_it_is_given() {
    let full = (1u128 << 127).to_string();

    assert_eq!(smoothed(5, "0"), iterated(0));
    assert_eq!(smoothed(5, &full), iterated(u32::MAX));
}

#[test]
fn iterated_price_should_move_down_when_price_falls() {
    let value = value(&falling(1));
    assert!(value < 0.75);
    assert!(value > 0.5);
}

#[test]
fn iterated_price_should_saturate_at_expected_staleness() {
    assert_ne!(iterated(4401), iterated(4402));
    assert_eq!(iterated(4402), iterated(10000));
}

#[test]
fn iterated_price_should_keep_equal_prices_unchanged() {
    let result = ema_iterated_price(
        "1".to_string(),
        "2".to_string(),
        "1".to_string(),
        "2".to_string(),
        50,
        TEN_MINUTES_SMOOTHING.to_string(),
    );
    assert_eq!(value(&result), 0.5);
}

#[test]
fn iterated_price_returns_error_on_invalid_input() {
    let result = ema_iterated_price(
        "invalid".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        1,
        TEN_MINUTES_SMOOTHING.to_string(),
    );
    assert_eq!(result, "-1");
}

#[test]
fn iterated_price_returns_error_on_invalid_smoothing() {
    let result = ema_iterated_price(
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        1,
        "invalid".to_string(),
    );
    assert_eq!(result, "-1");
}
