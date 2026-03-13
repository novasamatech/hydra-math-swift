use super::*;

fn two_asset_reserves() -> String {
    r#"[{"asset_id":0,"amount":"1000000000000","decimals":12},{"asset_id":1,"amount":"1000000000000","decimals":12}]"#.to_string()
}

fn two_asset_pegs() -> String {
    r#"[["1","1"],["1","1"]]"#.to_string()
}

// calculate_out_given_in

#[test]
fn out_given_in_should_work() {
    let result = stableswap_calculate_out_given_in(
        two_asset_reserves(),
        0,
        1,
        "1000000000".to_string(),
        "1".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_eq!(result, "999500248");
}

#[test]
fn out_given_in_with_fee_should_reduce_output() {
    let no_fee = stableswap_calculate_out_given_in(
        two_asset_reserves(),
        0,
        1,
        "1000000000".to_string(),
        "1".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    let with_fee = stableswap_calculate_out_given_in(
        two_asset_reserves(),
        0,
        1,
        "1000000000".to_string(),
        "1".to_string(),
        "0.01".to_string(),
        two_asset_pegs(),
    );
    let no_fee_amount: u128 = no_fee.parse().unwrap();
    let with_fee_amount: u128 = with_fee.parse().unwrap();
    assert!(with_fee_amount < no_fee_amount);
}

#[test]
fn out_given_in_returns_error_on_invalid_reserves() {
    let result = stableswap_calculate_out_given_in(
        "invalid".to_string(),
        0,
        1,
        "1000000000".to_string(),
        "1".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_eq!(result, "-1");
}

#[test]
fn out_given_in_returns_error_on_missing_asset() {
    let result = stableswap_calculate_out_given_in(
        two_asset_reserves(),
        0,
        99,
        "1000000000".to_string(),
        "1".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_eq!(result, "-1");
}

#[test]
fn out_given_in_returns_error_on_invalid_amount() {
    let result = stableswap_calculate_out_given_in(
        two_asset_reserves(),
        0,
        1,
        "not_a_number".to_string(),
        "1".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_eq!(result, "-1");
}

// calculate_in_given_out

#[test]
fn in_given_out_should_work() {
    let result = stableswap_calculate_in_given_out(
        two_asset_reserves(),
        0,
        1,
        "1000000000".to_string(),
        "1".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_ne!(result, "-1");
    let amount: u128 = result.parse().unwrap();
    assert!(amount > 0);
}

#[test]
fn in_given_out_with_fee_should_increase_input() {
    let no_fee = stableswap_calculate_in_given_out(
        two_asset_reserves(),
        0,
        1,
        "1000000000".to_string(),
        "1".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    let with_fee = stableswap_calculate_in_given_out(
        two_asset_reserves(),
        0,
        1,
        "1000000000".to_string(),
        "1".to_string(),
        "0.01".to_string(),
        two_asset_pegs(),
    );
    let no_fee_amount: u128 = no_fee.parse().unwrap();
    let with_fee_amount: u128 = with_fee.parse().unwrap();
    assert!(with_fee_amount > no_fee_amount);
}

#[test]
fn in_given_out_returns_error_on_invalid_reserves() {
    let result = stableswap_calculate_in_given_out(
        "invalid".to_string(),
        0,
        1,
        "1000000000".to_string(),
        "1".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_eq!(result, "-1");
}

// calculate_amplification

#[test]
fn amplification_should_interpolate() {
    let result = stableswap_calculate_amplification(
        "100".to_string(),
        "200".to_string(),
        "0".to_string(),
        "100".to_string(),
        "50".to_string(),
    );
    let amp: u128 = result.parse().unwrap();
    assert!(amp >= 100 && amp <= 200);
}

#[test]
fn amplification_at_start_should_equal_initial() {
    let result = stableswap_calculate_amplification(
        "100".to_string(),
        "200".to_string(),
        "0".to_string(),
        "100".to_string(),
        "0".to_string(),
    );
    assert_eq!(result, "100");
}

#[test]
fn amplification_at_end_should_equal_final() {
    let result = stableswap_calculate_amplification(
        "100".to_string(),
        "200".to_string(),
        "0".to_string(),
        "100".to_string(),
        "100".to_string(),
    );
    assert_eq!(result, "200");
}

#[test]
fn amplification_returns_error_on_invalid_input() {
    let result = stableswap_calculate_amplification(
        "not_a_number".to_string(),
        "200".to_string(),
        "0".to_string(),
        "100".to_string(),
        "50".to_string(),
    );
    assert_eq!(result, "-1");
}

// calculate_shares

#[test]
fn shares_should_work() {
    let reserves = r#"[{"asset_id":0,"amount":"90000000000","decimals":12},{"asset_id":1,"amount":"5000000000000000000000","decimals":12}]"#;
    let assets = r#"[{"asset_id":1,"amount":"43000000000000000000"}]"#;

    let result = stableswap_calculate_shares(
        reserves.to_string(),
        assets.to_string(),
        "1000".to_string(),
        "64839594451719860".to_string(),
        "0".to_string(),
        r#"[["1","1"],["1","1"]]"#.to_string(),
    );
    assert_eq!(result, "371541351762585");
}

#[test]
fn shares_returns_error_on_invalid_reserves() {
    let result = stableswap_calculate_shares(
        "invalid".to_string(),
        r#"[{"asset_id":0,"amount":"100"}]"#.to_string(),
        "100".to_string(),
        "1000".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_eq!(result, "-1");
}

#[test]
fn shares_returns_error_on_invalid_assets() {
    let result = stableswap_calculate_shares(
        two_asset_reserves(),
        "invalid".to_string(),
        "100".to_string(),
        "1000".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_eq!(result, "-1");
}

// calculate_shares_for_amount

#[test]
fn shares_for_amount_should_work() {
    let result = stableswap_calculate_shares_for_amount(
        two_asset_reserves(),
        0,
        "1000000000".to_string(),
        "100".to_string(),
        "2000000000000".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_ne!(result, "-1");
    let shares: u128 = result.parse().unwrap();
    assert!(shares > 0);
}

#[test]
fn shares_for_amount_returns_error_on_missing_asset() {
    let result = stableswap_calculate_shares_for_amount(
        two_asset_reserves(),
        99,
        "1000000000".to_string(),
        "100".to_string(),
        "2000000000000".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_eq!(result, "-1");
}

// calculate_add_one_asset

#[test]
fn add_one_asset_should_work() {
    let result = stableswap_calculate_add_one_asset(
        two_asset_reserves(),
        "500000000000".to_string(),
        0,
        "100".to_string(),
        "2000000000000".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_ne!(result, "-1");
    let amount: u128 = result.parse().unwrap();
    assert!(amount > 0);
}

#[test]
fn add_one_asset_returns_error_on_missing_asset() {
    let result = stableswap_calculate_add_one_asset(
        two_asset_reserves(),
        "500000000000".to_string(),
        99,
        "100".to_string(),
        "2000000000000".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_eq!(result, "-1");
}

// calculate_liquidity_out_one_asset

#[test]
fn liquidity_out_one_asset_should_work() {
    let result = stableswap_calculate_liquidity_out_one_asset(
        two_asset_reserves(),
        "500000000000".to_string(),
        0,
        "100".to_string(),
        "2000000000000".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_ne!(result, "-1");
    let amount: u128 = result.parse().unwrap();
    assert!(amount > 0);
}

#[test]
fn liquidity_out_one_asset_returns_error_on_missing_asset() {
    let result = stableswap_calculate_liquidity_out_one_asset(
        two_asset_reserves(),
        "500000000000".to_string(),
        99,
        "100".to_string(),
        "2000000000000".to_string(),
        "0".to_string(),
        two_asset_pegs(),
    );
    assert_eq!(result, "-1");
}

// invalid pegs

#[test]
fn out_given_in_returns_error_on_invalid_pegs() {
    let result = stableswap_calculate_out_given_in(
        two_asset_reserves(),
        0,
        1,
        "1000000000".to_string(),
        "1".to_string(),
        "0".to_string(),
        "invalid".to_string(),
    );
    assert_eq!(result, "-1");
}
