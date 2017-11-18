use std::io;

use fern;
use gotham::handler::NewHandlerService;
use hyper::server::Http;
use log::{self, LogLevelFilter};

use super::routing::router;

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        AuthService {}
    }

    fn set_logging(&self) {
        fern::Dispatch::new()
            .level(LogLevelFilter::Error)
            .level_for("gotham", log::LogLevelFilter::Trace)
            .level_for("gotham::state", log::LogLevelFilter::Error)
            .level_for("todo_session", log::LogLevelFilter::Error)
            .chain(io::stdout())
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}]{}",
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .apply()
            .unwrap();
    }

    pub fn run(&self) {
        self.set_logging();

        let addr = "127.0.0.1:7878".parse().unwrap();

        let server = Http::new()
            .bind(&addr, NewHandlerService::new(router()))
            .unwrap();

        server.run().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_service() {
        let _service = AuthService::new();
    }
}
