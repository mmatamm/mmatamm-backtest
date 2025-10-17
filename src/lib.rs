#![feature(sync_unsafe_cell)]
#![feature(btree_cursors)]

pub mod backtesting_market;
pub(crate) mod statistics;
pub mod stats_gathering_market;

pub(crate) mod ticker_hash;
