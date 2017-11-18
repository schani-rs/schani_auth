use hyper::server::{Request, Response};
use hyper::StatusCode;
use gotham::state::FromState;
use gotham::state::State;
use gotham::http::response::create_response;
use mime;

use super::extractors::{AuthenticateQueryString, VerifyRequestPath};
use super::super::service::auth;

pub fn authenticate(state: State, _req: Request) -> (State, Response) {
    let jwt = {
        let query: &AuthenticateQueryString = AuthenticateQueryString::borrow_from(&state);
        auth::authenticate(auth::Credentials::from(query))
    };

    if let Ok(jwt) = jwt {
        let resp = create_response(
            &state,
            StatusCode::Ok,
            Some((jwt.into_bytes(), mime::TEXT_PLAIN)),
        );
        (state, resp)
    } else {
        let resp = create_response(&state, StatusCode::BadRequest, None);
        (state, resp)
    }
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
