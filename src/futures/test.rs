
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