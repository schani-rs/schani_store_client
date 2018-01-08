extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate tokio_core;

mod client;

pub use client::StoreClient;
