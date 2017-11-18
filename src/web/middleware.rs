use std::io;

use gotham;
use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::State;

use hyper::Request;

use futures_cpupool::CpuPool;

pub struct ThreadPoolMiddleware {
    pool: CpuPool,
}

impl ThreadPoolMiddleware {
    pub fn new(pool: CpuPool) -> Self {
        ThreadPoolMiddleware { pool: pool }
    }
}

impl Middleware for ThreadPoolMiddleware {
    fn call<Chain>(self, mut state: State, request: Request, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State, Request) -> Box<HandlerFuture> + Send + 'static,
        Self: Sized,
    {
        state.put(ThreadPoolMiddlewareData::new(self.pool.clone()));

        chain(state, request)
    }
}

impl NewMiddleware for ThreadPoolMiddleware {
    type Instance = ThreadPoolMiddleware;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(ThreadPoolMiddleware {
            pool: self.pool.clone(),
        })
    }
}

#[derive(StateData)]
pub struct ThreadPoolMiddlewareData {
    pool: CpuPool,
}

impl ThreadPoolMiddlewareData {
    pub fn new(pool: CpuPool) -> Self {
        ThreadPoolMiddlewareData { pool: pool }
    }
}

impl ThreadPoolMiddlewareData {
    pub fn get_pool(&self) -> CpuPool {
        self.pool.clone()
    }
}
