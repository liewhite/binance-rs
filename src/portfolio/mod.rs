
use std::time::{SystemTime, UNIX_EPOCH};


use crate::api::Binance;


pub mod account;
pub mod general;
pub mod market;
pub mod model;
pub mod userstream;
pub mod websockets;

#[test]
fn test_papi() {
    let cli: account::PortfolioAccount = Binance::new(
        Some("".to_string()),
        Some("".to_string()),
    );
    let a = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - 86400 * 1000;
    let trades = cli.get_user_trades("", None,Some(a as u64),None,None).unwrap();
    trades.iter().for_each(|t| {
        println!("{:?} {:?} {:?}", t.symbol,t.qty * t.price,t.time);

    });
    println!("{:?}", trades);
}
