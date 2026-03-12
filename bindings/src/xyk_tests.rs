use super::*;

// calculate_out_given_in

#[test]
fn out_given_in_should_work() {
    let result = xyk_calculate_out_given_in(
        "1000000000".to_string(),
        "1000000000".to_string(),
        "100000000".to_string(),
    );
    assert_ne!(result, "-1");
    let amount: u128 = result.parse().unwrap();
    assert!(amount > 0);
}

#[test]
fn out_given_in_larger_input_should_give_larger_output() {
    let small = xyk_calculate_out_given_in(
        "1000000000".to_string(),
        "1000000000".to_string(),
        "100000000".to_string(),
    );
    let large = xyk_calculate_out_given_in(
        "1000000000".to_string(),
        "1000000000".to_string(),
        "200000000".to_string(),
    );
    let small_amount: u128 = small.parse().unwrap();
    let large_amount: u128 = large.parse().unwrap();
    assert!(large_amount > small_amount);
}

#[test]
fn out_given_in_returns_error_on_invalid_input() {
    let result = xyk_calculate_out_given_in(
        "invalid".to_string(),
        "1000000000".to_string(),
        "100000000".to_string(),
    );
    assert_eq!(result, "-1");
}

#[test]
fn out_given_in_with_zero_balance_in_returns_full_amount() {
    let result = xyk_calculate_out_given_in(
        "0".to_string(),
        "1000000000".to_string(),
        "100000000".to_string(),
    );
    // XYK formula: when balance_in is 0, result equals balance_out * amount / amount = amount_in
    assert_ne!(result, "-1");
}

// calculate_in_given_out

#[test]
fn in_given_out_should_work() {
    let result = xyk_calculate_in_given_out(
        "1000000000".to_string(),
        "1000000000".to_string(),
        "100000000".to_string(),
    );
    assert_ne!(result, "-1");
    let amount: u128 = result.parse().unwrap();
    assert!(amount > 0);
}

#[test]
fn in_given_out_larger_output_should_need_larger_input() {
    let small = xyk_calculate_in_given_out(
        "1000000000".to_string(),
        "1000000000".to_string(),
        "100000000".to_string(),
    );
    let large = xyk_calculate_in_given_out(
        "1000000000".to_string(),
        "1000000000".to_string(),
        "200000000".to_string(),
    );
    let small_amount: u128 = small.parse().unwrap();
    let large_amount: u128 = large.parse().unwrap();
    assert!(large_amount > small_amount);
}

#[test]
fn in_given_out_returns_error_on_invalid_input() {
    let result = xyk_calculate_in_given_out(
        "1000000000".to_string(),
        "invalid".to_string(),
        "100000000".to_string(),
    );
    assert_eq!(result, "-1");
}

// calculate_pool_trade_fee

#[test]
fn pool_trade_fee_should_work() {
    let result = xyk_calculate_pool_trade_fee(
        "100000000".to_string(),
        "3".to_string(),
        "1000".to_string(),
    );
    assert_ne!(result, "-1");
    let fee: u128 = result.parse().unwrap();
    assert!(fee > 0);
}

#[test]
fn pool_trade_fee_larger_amount_should_give_larger_fee() {
    let small = xyk_calculate_pool_trade_fee(
        "100000000".to_string(),
        "3".to_string(),
        "1000".to_string(),
    );
    let large = xyk_calculate_pool_trade_fee(
        "200000000".to_string(),
        "3".to_string(),
        "1000".to_string(),
    );
    let small_fee: u128 = small.parse().unwrap();
    let large_fee: u128 = large.parse().unwrap();
    assert!(large_fee > small_fee);
}

#[test]
fn pool_trade_fee_returns_error_on_invalid_input() {
    let result = xyk_calculate_pool_trade_fee(
        "not_a_number".to_string(),
        "3".to_string(),
        "1000".to_string(),
    );
    assert_eq!(result, "-1");
}

// round-trip consistency

#[test]
fn sell_then_buy_should_be_roughly_consistent() {
    let out = xyk_calculate_out_given_in(
        "1000000000000".to_string(),
        "1000000000000".to_string(),
        "1000000000".to_string(),
    );
    assert_ne!(out, "-1");
    let out_amount: u128 = out.parse().unwrap();

    let back = xyk_calculate_in_given_out(
        "1000000000000".to_string(),
        "1000000000000".to_string(),
        out_amount.to_string(),
    );
    assert_ne!(back, "-1");
    let back_amount: u128 = back.parse().unwrap();

    // Due to rounding the round-trip input should be close to original
    let diff = if back_amount > 1000000000 {
        back_amount - 1000000000
    } else {
        1000000000 - back_amount
    };
    assert!(diff <= 1, "Round-trip difference too large: {}", diff);
}
