use crypto::sha2::Sha256;
use jwt;

#[derive(Debug)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn new(username: String, password: String) -> Self {
        Credentials {
            username: username,
            password: password,
        }
    }
}

// #[post("/authenticate", data = "<credentials>")]
pub fn authenticate(credentials: Credentials) -> Result<String, &'static str> {
    // TODO: verify against userinfo service
    if credentials.password != "123456" {
        return Err("invalid username/password");
    }

    let header = jwt::Header::default();
    let claims = jwt::Registered {
        iss: Some("schani-rs".into()),
        sub: Some(credentials.username.to_owned()),
        ..Default::default()
    };
    let token = jwt::Token::new(header, claims);

    token
        .signed(b"secret", Sha256::new())
        .map_err(|_| "could not sign JWT")
}

// #[post("/verify/<token>")]
pub fn verify(token: &str) -> Result<(), &'static str> {
    let token = try!(
        jwt::Token::<jwt::Header, jwt::Registered>::parse(token)
            .map_err(|_| "could not parse token")
    );
    if token.verify(b"secret", Sha256::new()) {
        Ok(())
    } else {
        Err("token verification failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_creds() {
        let creds = Credentials::new("christoph".to_string(), "password".to_string());

        assert_eq!("christoph", creds.username);
        assert_eq!("password", creds.password);
    }

    #[test]
    fn test_failing_authentication() {
        let creds = Credentials {
            username: "christoph".to_owned(),
            password: "wrong".to_owned(),
        };

        let result = authenticate(creds);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "invalid username/password");
    }

    #[test]
    fn test_successful_authentication() {
        let creds = Credentials {
            username: "christoph".to_owned(),
            password: "123456".to_owned(),
        };

        let result = authenticate(creds);

        assert!(result.is_ok());
    }

    #[test]
    fn test_jwt_parsing_error() {
        let jwt = "notreallyajwt";

        let result = verify(jwt);

        assert!(result.is_err());
    }

    #[test]
    fn test_verification() {
        let creds = Credentials {
            username: "christoph".to_owned(),
            password: "123456".to_owned(),
        };
        let jwt = authenticate(creds).unwrap();

        let result = verify(&jwt);

        assert!(result.is_ok());
    }
}
