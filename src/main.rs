extern crate schani_auth;

use schani_auth::AuthService;

fn main() {
    let service = AuthService::new();

    service.run();
}
