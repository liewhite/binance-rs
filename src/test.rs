use std::time::{SystemTime, UNIX_EPOCH};

use crate::{api::Binance, savings::Savings};

#[test]
fn test_loan_data() {
    let cli: Savings = Binance::new(
        Some("".to_string()),
        Some("".to_string()),
    );
    let a = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        - 86400 * 1000;
    let data = cli
        .get_loan_data("USDC")
        .unwrap();
    println!("{:?}", data);
}
