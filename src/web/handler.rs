use futures::Future;
use futures::future::{err, ok};
use hyper::server::{Request, Response};
use hyper::StatusCode;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::http::response::create_response;
use gotham::state::FromState;
use gotham::state::State;
use mime;

use super::extractors::{AuthenticateQueryString, VerifyRequestPath};
use super::middleware::ThreadPoolMiddlewareData;
use super::super::service::auth;

pub fn authenticate(state: State, _req: Request) -> Box<HandlerFuture> {
    let work = {
        let query: &AuthenticateQueryString = AuthenticateQueryString::borrow_from(&state);
        let pool: &ThreadPoolMiddlewareData = state.borrow::<ThreadPoolMiddlewareData>().unwrap();
        let creds = auth::Credentials::from(query);
        pool.get_pool().spawn_fn(move || auth::authenticate(creds))
    };

    let result = work.then(|result| match result {
        Ok(jwt) => {
            let resp = create_response(
                &state,
                StatusCode::Ok,
                Some((jwt.into_bytes(), mime::TEXT_PLAIN)),
            );
            ok((state, resp))
        }
        Err(e) => err((state, e.into_handler_error())),
    });

    Box::new(result)
}

pub fn verify(state: State, _req: Request) -> (State, Response) {
    let valid = {
        let jwt: &VerifyRequestPath = VerifyRequestPath::borrow_from(&state);
        auth::verify(&jwt.jwt).is_ok()
    };

    if valid {
        let resp = create_response(&state, StatusCode::Ok, None);
        (state, resp)
    } else {
        let resp = create_response(&state, StatusCode::BadRequest, None);
        (state, resp)
    }
}
