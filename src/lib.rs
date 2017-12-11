#[macro_use]
extern crate error_chain;
extern crate fern;
extern crate futures;
extern crate futures_cpupool;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate log;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod error;
mod service;
mod web;

pub use web::webservice::AuthService;
