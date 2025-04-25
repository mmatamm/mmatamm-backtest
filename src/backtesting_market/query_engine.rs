mod timestep_series;

use ahash::{HashMap, HashMapExt};
use chrono::DateTime;
use dashmap::DashMap;
use timestep_series::TimestepSeries;

use mmatamm_interface::market::SystemEvent;
use tokio::sync::RwLock;

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
    fetcher: RwLock<F>,

    // TODO PERF Use ahash::RandomState
    // prices_series: DashMap<String, TimestepSeries<Ohlc>, ahash::RandomState>,
    prices_series: DashMap<String, TimestepSeries<Ohlc>>,
    system_events_series: RwLock<TimestepSeries<SystemEvent>>,
}

// impl std::fmt::Display for QueryEngine {
//     fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

impl<F: Fetcher> QueryEngine<F> {
    pub async fn new(fetcher: RwLock<F>) -> Result<Self, F::Error> {
        let system_events_series =
            TimestepSeries::new(fetcher.read().await.fetch_system_events().await?);

        Ok(Self {
            fetcher,
            prices_series: DashMap::new(),
            system_events_series: RwLock::new(system_events_series),
        })
    }

    pub async fn query_price(
        &self,
        time: &chrono::DateTime<chrono::Utc>,
        symbol: &str,
    ) -> Result<Option<f32>, F::Error> {
        if !self.prices_series.contains_key(symbol) {
            let s = TimestepSeries::new(
                self.fetcher
                    .read()
                    .await
                    .fetch_ticker_prices(symbol)
                    .await?,
            );
            self.prices_series.insert(symbol.to_owned(), s);
        }

        let series = self.prices_series.get(symbol).unwrap();

        Ok(series
            .query_before(&time.naive_utc())
            .map(|(_time, ohlc)| ohlc.close))
    }

    pub async fn query_system_event(
        &self,
        time: &chrono::DateTime<chrono::Utc>,
    ) -> Option<(SystemEvent, chrono::NaiveDateTime)> {
        // TODO use NaiveDateTime
        let system_events_series = self.system_events_series.read().await;
        let event_opt = system_events_series.query_after(&time.naive_utc());
        event_opt.map(|(timestamp, event)| {
            (
                event.clone(),
                DateTime::from_timestamp(*timestamp, 0).unwrap().naive_utc(),
            )
        })
    }
}
