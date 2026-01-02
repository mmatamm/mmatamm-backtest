use std::{error::Error, sync::Arc};

use chrono::{DateTime, TimeDelta, Utc};
use mmatamm_backtest::{
    backtesting_market::{
        BacktestingMarket, QueryEngine,
        fetcher::{HDF5Fetcher, QuestDbFetcher},
    },
    stats_gathering_market::StatsGatheringMarket,
};
use mmatamm_interface::{AnyhowMarket, external_algorithm::ExternalAlgorithm};
use postgres::NoTls;

fn main() -> Result<(), Box<dyn Error>> {
    flexi_logger::Logger::try_with_env()
        .unwrap()
        .start()
        .unwrap();

    // Connect to the database
    // let db_client = postgres::Client::connect(
    //     "user=admin password=quest host=localhost port=8812 dbname=qdb",
    //     NoTls,
    // )?;

    // let fetcher = QuestDbFetcher::new(db_client)?;
    let fetcher = HDF5Fetcher::new("/home/user/data/month-of-prices.h5")?;
    let query_engine = Arc::new(QueryEngine::new(fetcher)?);

    let backtesting_market = BacktestingMarket::new(
        query_engine,
        "2024-06-25T13:00:00Z".parse::<DateTime<Utc>>()?.into(),
        10_000.0,
    )?;
    let mut stats_market = StatsGatheringMarket::new(
        backtesting_market,
        TimeDelta::minutes(15),
        "SPY".to_string(),
    );

    let mut anyhow_market = AnyhowMarket::new(&mut stats_market);

    let mut algo = unsafe {
        ExternalAlgorithm::new(
            "/home/user/moving_average_crossing/bazel-bin/main/libmoving_average_crossing.so",
        )
    }?;

    algo.run(&mut anyhow_market, &[60.0, 5.0, 20.0])?;
    // let mut myalgo = CrossMovingAverageStrategy::new("AAPL", TimeDelta::seconds(948), 10, 13);
    // myalgo.run(&mut market)?;

    println!("{}", stats_market.display()?);

    Ok(())
}
