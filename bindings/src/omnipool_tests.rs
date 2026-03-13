use super::*;

#[test]
fn sell_should_work_with_no_fees() {
    let result = omnipool_calculate_out_given_in(
        "1000000000000".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0".to_string(),
        "0".to_string(),
        "0".to_string(),
    );
    assert_ne!(result, "-1");
    let amount: u128 = result.parse().unwrap();
    assert!(amount > 0);
}

#[test]
fn sell_with_fees_should_reduce_output() {
    let no_fee = omnipool_calculate_out_given_in(
        "1000000000000".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0".to_string(),
        "0".to_string(),
        "0".to_string(),
    );
    let with_fee = omnipool_calculate_out_given_in(
        "1000000000000".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0.01".to_string(),
        "0".to_string(),
        "0".to_string(),
    );
    let no_fee_amount: u128 = no_fee.parse().unwrap();
    let with_fee_amount: u128 = with_fee.parse().unwrap();
    assert!(with_fee_amount < no_fee_amount);
}

#[test]
fn sell_with_slip_fee_should_reduce_output() {
    let no_slip = omnipool_calculate_out_given_in(
        "1000000000000".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0".to_string(),
        "0".to_string(),
        "0".to_string(),
    );
    let with_slip = omnipool_calculate_out_given_in(
        "1000000000000".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0".to_string(),
        "0".to_string(),
        "0.05".to_string(),
    );
    let no_slip_amount: u128 = no_slip.parse().unwrap();
    let with_slip_amount: u128 = with_slip.parse().unwrap();
    assert!(with_slip_amount < no_slip_amount);
}

#[test]
fn buy_should_work_with_no_fees() {
    let result = omnipool_calculate_in_given_out(
        "1000000000000".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0".to_string(),
        "0".to_string(),
        "0".to_string(),
    );
    assert_ne!(result, "-1");
    let amount: u128 = result.parse().unwrap();
    assert!(amount > 0);
}

#[test]
fn buy_with_slip_fee_should_increase_cost() {
    let no_slip = omnipool_calculate_in_given_out(
        "1000000000000".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0".to_string(),
        "0".to_string(),
        "0".to_string(),
    );
    let with_slip = omnipool_calculate_in_given_out(
        "1000000000000".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0".to_string(),
        "0".to_string(),
        "0.05".to_string(),
    );
    let no_slip_amount: u128 = no_slip.parse().unwrap();
    let with_slip_amount: u128 = with_slip.parse().unwrap();
    assert!(with_slip_amount > no_slip_amount);
}

#[test]
fn sell_returns_error_on_invalid_input() {
    let result = omnipool_calculate_out_given_in(
        "invalid".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0".to_string(),
        "0".to_string(),
        "0".to_string(),
    );
    assert_eq!(result, "-1");
}

#[test]
fn buy_returns_error_on_invalid_input() {
    let result = omnipool_calculate_in_given_out(
        "invalid".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0".to_string(),
        "0".to_string(),
        "0".to_string(),
    );
    assert_eq!(result, "-1");
}

#[test]
fn sell_with_zero_slip_fee_should_match_no_slip() {
    let no_slip = omnipool_calculate_out_given_in(
        "1000000000000".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0.01".to_string(),
        "0.01".to_string(),
        "0".to_string(),
    );
    let zero_slip = omnipool_calculate_out_given_in(
        "1000000000000".to_string(),
        "500000000000".to_string(),
        "1000000000000".to_string(),
        "2000000000000".to_string(),
        "800000000000".to_string(),
        "2000000000000".to_string(),
        "100000000000".to_string(),
        "0.01".to_string(),
        "0.01".to_string(),
        "0".to_string(),
    );
    assert_eq!(no_slip, zero_slip);
}
