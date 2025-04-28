mod timestep_series;

use std::sync::{Arc, Mutex};

use chrono::DateTime;
use papaya;
use timestep_series::TimestepSeries;

use mmatamm_interface::market::SystemEvent;

use super::Fetcher;

#[derive(Clone, Debug)]
pub struct Ohlc {
    // TODO PERF I don't need the open price. Maybe I don't even need the close price.
    // TODO PERF Maybe storing the rest of the values relatively to open is a good idea,
    // computation is cheaper than memory...
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}

#[derive(Debug)]
pub struct QueryEngine<F: Fetcher> {
    fetcher: Arc<Mutex<F>>,

    prices_series: papaya::HashMap<String, TimestepSeries<Ohlc>, ahash::RandomState>,
    system_events_series: Mutex<TimestepSeries<SystemEvent>>,
}

impl<F: Fetcher> QueryEngine<F> {
    pub fn new(mut fetcher: F) -> Result<Self, F::Error> {
        let system_events_series = TimestepSeries::new(fetcher.fetch_system_events()?);
        let fetcher_mutex = Arc::new(Mutex::new(fetcher));

        Ok(Self {
            fetcher: fetcher_mutex,
            prices_series: papaya::HashMap::with_hasher(ahash::RandomState::new()),
            system_events_series: Mutex::new(system_events_series),
        })
    }

    pub fn query_price(
        &self,
        time: &chrono::DateTime<chrono::Utc>,
        symbol: &str,
    ) -> Result<Option<f32>, F::Error> {
        if !self.prices_series.pin().contains_key(symbol) {
            let s = TimestepSeries::new(self.fetcher.lock().unwrap().fetch_ticker_prices(symbol)?);
            self.prices_series.pin().insert(symbol.to_string(), s);
        }

        let prices_series = self.prices_series.pin();
        let series = prices_series.get(symbol).unwrap();

        Ok(series
            .query_before(&time.naive_utc())
            .map(|(_time, ohlc)| ohlc.close))
    }
    // pub fn query_price(
    //     &self,
    //     time: &chrono::DateTime<chrono::Utc>,
    //     symbol: &str,
    // ) -> Result<Option<f32>, F::Error> {
    //     if !self.prices_series.contains_key(symbol) {
    //         let s = TimestepSeries::new(self.fetcher.lock().unwrap().fetch_ticker_prices(symbol)?);
    //         self.prices_series.insert(symbol.to_string(), s);
    //     }

    //     let series = self.prices_series.get(symbol).unwrap();

    //     Ok(series
    //         .query_before(&time.naive_utc())
    //         .map(|(_time, ohlc)| ohlc.close))
    // }

    pub fn query_system_event(
        &self,
        time: &chrono::DateTime<chrono::Utc>,
    ) -> Option<(SystemEvent, chrono::NaiveDateTime)> {
        // TODO use NaiveDateTime
        let system_events_series = self.system_events_series.lock().unwrap();
        let event_opt = system_events_series.query_after(&time.naive_utc());
        event_opt.map(|(timestamp, event)| {
            (
                event.clone(),
                DateTime::from_timestamp(*timestamp, 0).unwrap().naive_utc(),
            )
        })
    }
}
