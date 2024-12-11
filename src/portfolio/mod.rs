use account::IncomeRequest;

use crate::api::Binance;

pub mod account;
pub mod general;
pub mod market;
pub mod model;
pub mod userstream;
pub mod websockets;

#[test]
fn test_position_risk() {
    let cli: account::PortfolioAccount = Binance::new(
        Some("".to_string()),
        Some("".to_string()),
    );
    // let pos = cli.position_information().unwrap();
    // println!("{:?}", pos);
    // // cli.market_buy("BTCUSDT", 0.002).unwrap();
    // cli.market_sell("BTCUSDT", 0.002).unwrap();
    // let b = cli.account_balance("USDT").unwrap();
    // println!("{:?}", b.total_wallet_balance + b.um_unrealized_pNL)

    let income = cli.get_income(IncomeRequest {
        symbol: Some("BTCUSDT".to_string()),
        income_type: Some(account::IncomeType::COMMISSION),
        start_time: None,
        end_time: None,
        limit: Some(1000_u32),
    }).unwrap();
    let total: f64 = income.iter().map(|x|x.income).sum();
    println!("{:?}", total)
}
