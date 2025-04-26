
#[test]
pub fn test_get_continuous_klines() {
    use crate::{api::Binance, futures::market::FuturesMarket, model::KlineSummaries};

    let cli: FuturesMarket = Binance::new(None, None);
    let klines = cli.get_continuous_klines("ETHUSDT", "1d", Some(7), None, None).unwrap();
    match klines {
        KlineSummaries::AllKlineSummaries(klines) => {
            for kline in klines {
                println!("{:?}", kline);
            }
        }
    }
}

#[test]
pub fn test_get_funding_rate() {
    use crate::{api::Binance, futures::market::FuturesMarket};

    let cli: FuturesMarket = Binance::new(None, None);
    let funding_rate = cli.get_funding_rate("ETHUSDT", 10).unwrap();
    for rate in funding_rate {
        println!("ts: {}, rate: {}", rate.funding_time, rate.funding_rate);
    }
}