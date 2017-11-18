use std::convert::From;

use gotham;
use hyper;

use super::super::service::auth::Credentials;

#[derive(StateData, FromState, QueryStringExtractor, StaticResponseExtender)]
pub struct AuthenticateQueryString {
    pub username: String,
    pub password: String,
}

impl<'a> From<&'a AuthenticateQueryString> for Credentials {
    fn from(query: &AuthenticateQueryString) -> Self {
        Credentials::new(query.username.to_owned(), query.password.to_owned())
    }
}

#[derive(StateData, FromState, PathExtractor, StaticResponseExtender)]
pub struct VerifyRequestPath {
    pub jwt: String,
}
