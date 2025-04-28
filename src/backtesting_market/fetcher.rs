mod hdf5_fetcher;
mod questdb_fetcher;

use std::{error::Error as StdError, fmt::Display};

use super::query_engine::Ohlc;
pub use hdf5_fetcher::HDF5Fetcher;
pub use questdb_fetcher::QuestDbFetcher;

use mmatamm_interface::market::SystemEvent;

// TODO XXX Return the timestamp of the relevant row
// TODO The `: Display` shouldn't be here

pub trait Fetcher: Display {
    type Error: StdError + Send;

    fn fetch_system_events(&mut self) -> Result<Vec<(i64, SystemEvent)>, Self::Error>;
    fn fetch_ticker_prices(&mut self, symbol: &str) -> Result<Vec<(i64, Ohlc)>, Self::Error>;
}
