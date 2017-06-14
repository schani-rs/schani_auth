#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate crypto;
extern crate jwt;
extern crate rocket;
extern crate rocket_contrib;

use crypto::sha2::Sha256;
use std::result::Result;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::status::Custom;

#[derive(Debug, FromForm)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/authenticate", data = "<credentials>")]
fn authenticate(credentials: Form<Credentials>) -> Result<String, Custom<&'static str>> {
    let creds: &Credentials = credentials.get();

    // TODO: verify against userinfo service
    if creds.password != "123456" {
        return Err(Custom(Status::Unauthorized, "invalid username/password"));
    }

    let header = jwt::Header::default();
    let claims = jwt::Registered {
        iss: Some("schani-rs".into()),
        sub: Some(creds.username.to_owned()),
        ..Default::default()
    };
    let token = jwt::Token::new(header, claims);

    Ok(token.signed(b"secret", Sha256::new()).expect("to workd"))
}

#[post("/verify/<token>")]
fn verify(token: &str) -> Result<(), Custom<&'static str>> {
    let token = try!(jwt::Token::<jwt::Header, jwt::Registered>::parse(token)
                         .map_err(|_| Custom(Status::BadRequest, "could not parse token")));
    if token.verify(b"secret", Sha256::new()) {
        Ok(())
    } else {
        Err(Custom(Status::Unauthorized, "token verification failed"))
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![authenticate, verify])
        .launch();
}
