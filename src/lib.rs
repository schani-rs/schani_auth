extern crate crypto;
extern crate fern;
extern crate futures;
extern crate futures_cpupool;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate jwt;
#[macro_use]
extern crate log;
extern crate mime;

mod service;
mod web;

pub use web::webservice::AuthService;
