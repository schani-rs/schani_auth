use std::io;

use fern;
use futures_cpupool::CpuPool;
use gotham::handler::NewHandlerService;
use hyper::server::Http;
use log::{self, LevelFilter};

use super::routing::router;

pub struct AuthService {
    pool: CpuPool,
}

impl AuthService {
    pub fn new() -> Self {
        AuthService {
            pool: CpuPool::new_num_cpus(),
        }
    }

    fn set_logging(&self) {
        fern::Dispatch::new()
            .level(LevelFilter::Error)
            .level_for("gotham", log::LevelFilter::Trace)
            .level_for("gotham::state", log::LevelFilter::Error)
            .level_for("todo_session", log::LevelFilter::Error)
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

        let addr = "0.0.0.0:8000".parse().unwrap();

        let server = Http::new()
            .bind(&addr, NewHandlerService::new(router(&self.pool)))
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
