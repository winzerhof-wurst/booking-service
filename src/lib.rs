extern crate fern;
extern crate futures;
extern crate futures_state_stream;
extern crate gotham;
extern crate gotham_middleware_postgres;
extern crate gotham_rest;
extern crate gotham_serde_json_body_parser;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;

mod handlers;
mod models;
mod service;

pub use service::BookingWebService;
