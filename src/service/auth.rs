use jwt;

use error::{self, ResultExt};

#[derive(Debug)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
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
pub fn authenticate(credentials: Credentials) -> error::Result<String> {
    // TODO: verify against userinfo service
    if credentials.password != "123456" {
        return Err("invalid username/password".into());
    }

    let header = jwt::Header::default();
    let claims = Claims {
        sub: credentials.username.to_owned(),
        company: "schani-rs".to_owned(),
    };
    jwt::encode(&header, &claims, "secret".as_ref()).chain_err(|| "could not sign JWT")
}

// #[post("/verify/<token>")]
pub fn verify(token: &str) -> error::Result<()> {
    jwt::decode::<Claims>(&token, "secret".as_ref(), &jwt::Validation::default())
        .map(|_| ())
        .chain_err(|| "could not parse token")
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
