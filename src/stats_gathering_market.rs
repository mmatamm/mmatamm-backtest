// TODO Make `display` and tabled dependency optional

use chrono::{DateTime, DurationRound, TimeDelta, Utc};
use tabled::{builder::Builder, settings::Style};

use mmatamm_interface::market::{Event, Market, MarketTime};

use crate::statistics::StdDevAggregator;

/// A `Market` wrapper that gathers statistics about a strategy
pub struct StatsGatheringMarket<M: Market + Send> {
    market: M,

    sample_rate: TimeDelta,
    benchmark: String,

    next_sample: DateTime<Utc>,

    first_trade: Option<DateTime<Utc>>,
    initial_net_worth: Option<f32>,
    initial_benchmark: Option<f32>,

    // This is useful for calculating the Sharpe ratio
    excess_returns_aggregator: StdDevAggregator,
}

impl<M: Market + Send> StatsGatheringMarket<M> {
    pub fn new(market: M, sample_rate: TimeDelta, benchmark: String) -> Self {
        let next_sample = market.time().duration_trunc(sample_rate).unwrap() + sample_rate;

        StatsGatheringMarket {
            market,

            sample_rate,
            benchmark,

            next_sample,

            first_trade: None,
            initial_net_worth: None,
            initial_benchmark: None,

            excess_returns_aggregator: StdDevAggregator::default(),
        }
    }

    pub fn first_trade(&self) -> Option<DateTime<Utc>> {
        self.first_trade
    }

    pub fn duration(&self) -> Option<TimeDelta> {
        Some(self.market.time() - self.first_trade?)
    }

    pub fn net_return(&self) -> Result<Option<f32>, M::Error> {
        self.market
            .net_worth()
            .map(|current| Some(current / self.initial_net_worth? - 1.0))
    }

    pub fn sharpe_ratio(&self) -> Option<f32> {
        if let Some(stddev) = self.excess_returns_aggregator.calc_stddev(true) {
            let mean = self
                .excess_returns_aggregator
                .calc_mean()
                .expect("the standard deviation is known yet the mean is not");

            Some(mean / stddev)
        } else {
            None
        }
    }

    pub fn display(&self) -> Result<String, M::Error> {
        let mut builder = Builder::default();

        builder.push_record(["Current Time", &format!("{}", self.market.time())]);
        if let Some(first_trade) = self.first_trade {
            builder.push_record(["First Trade", &format!("{}", first_trade)]);
        }

        if let Some(duration) = self.duration() {
            builder.push_record(["Duration (Days)", &duration.num_days().to_string()]);
        }

        if let Some(net_return) = self.net_return()? {
            builder.push_record(["Net Return", &format!("{0:.2}%", net_return * 100.0)]);
        }

        if let Some(sharpe_ratio) = self.sharpe_ratio() {
            builder.push_record(["Sharpe Ratio", &format! {"{0:.2}", sharpe_ratio}]);
        }

        let mut table = builder.build();
        table.with(Style::modern());

        Ok(table.to_string())
    }

    fn take_sample(&mut self) {
        let net_worth = self.net_worth().unwrap() as f32;
        let benchmark = self.current_price(&self.benchmark).unwrap() as f32;

        if let Some(previous_net_worth) = self.initial_net_worth {
            let previous_benchmark = self
                .initial_benchmark
                .expect("previous_net_worth is set but previous_benchmark is not");

            let excess = net_worth / previous_net_worth - benchmark / previous_benchmark;
            self.excess_returns_aggregator.update(excess);
        } else {
            self.initial_net_worth = Some(net_worth);
            self.initial_benchmark = Some(benchmark);
        }
    }
}

impl<M: Market + Send> Market for StatsGatheringMarket<M> {
    type Error = M::Error;

    fn next_event(&mut self) -> Result<Option<(DateTime<Utc>, Event)>, Self::Error> {
        loop {
            let (time, event) = self.market.next_event_until(self.next_sample)?;

            match event {
                Event::Deadline => {
                    self.next_sample += self.sample_rate;
                    self.take_sample();
                    // Continue the loop to fetch the next event
                }
                e => return Ok(Some((time, e))),
            }
        }
    }

    // async fn next_event_until(
    //     &mut self,
    //     deadline: chrono::DateTime<Utc>,
    // ) -> Result<(DateTime<Utc>, Event), Self::Error> {
    //     loop {
    //         match self.next_sample.cmp(&deadline) {
    //             Ordering::Less => {
    //                 let (time, event) = self.market.next_event_until(self.next_sample).await?;
    //                 match event {
    //                     Event::Deadline => {
    //                         self.next_sample += self.sample_rate;
    //                         self.take_sample();
    //                         // Continue the loop to fetch the next event
    //                     }
    //                     e => return Ok((time, e)),
    //                 }
    //                 // Continue the loop to fetch the next event
    //             }
    //             Ordering::Equal => {
    //                 let (time, event) = self.market.next_event_until(self.next_sample).await?;
    //                 if event == Event::Deadline {
    //                     self.next_sample += self.sample_rate;
    //                     self.take_sample();
    //                 }
    //                 return Ok((time, event));
    //             }
    //             Ordering::Greater => return self.market.next_event_until(deadline).await,
    //         }
    //     }
    // }
    fn next_event_until(
        &mut self,
        deadline: chrono::DateTime<Utc>,
    ) -> Result<(DateTime<Utc>, Event), Self::Error> {
        loop {
            let until = self.next_sample.min(deadline);
            let (time, event) = self.market.next_event_until(until)?;

            match event {
                Event::Deadline if self.next_sample <= deadline => {
                    self.next_sample += self.sample_rate;
                    self.take_sample();
                    // Continue the loop to fetch the next event
                }
                _ => return Ok((time, event)),
            }
        }
    }

    fn time(&self) -> DateTime<Utc> {
        self.market.time()
    }

    fn price_at(&self, symbol: &str, time: DateTime<Utc>) -> Result<f32, Self::Error> {
        self.market.price_at(symbol, time)
    }

    fn buy_at_market(&mut self, symbol: &str, quantity: u32) -> Result<(), Self::Error> {
        if self.first_trade.is_none() {
            self.first_trade = Some(self.market.time());
        }

        self.market.buy_at_market(symbol, quantity)
    }

    fn sell_at_market(&mut self, symbol: &str, quantity: u32) -> Result<(), Self::Error> {
        self.market.sell_at_market(symbol, quantity)
    }

    fn market_time(&self) -> MarketTime {
        self.market.market_time()
    }

    fn cash(&self) -> f32 {
        self.market.cash()
    }

    fn shares_of(&self, symbol: &str) -> u32 {
        self.market.shares_of(symbol)
    }

    fn holdings(&self) -> impl IntoIterator<Item = (&String, &u32)> {
        self.market.holdings()
    }
}

// Max. Drawdown
// Avg. Drawdown
// Start, End & Duration
// Equity Final & Equity Peak
// Return & Ann. Return
// Ann. Volatility
// Sharpe Ratio
// Sortino Ratio
// Calmar Ratio
