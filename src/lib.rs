extern crate fern;
extern crate futures;
extern crate gotham;
extern crate gotham_rest;
extern crate gotham_serde_json_body_parser;
extern crate log;

mod handlers;
mod service;

pub use service::BookingWebService;
