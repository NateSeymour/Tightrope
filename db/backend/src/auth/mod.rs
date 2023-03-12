use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::Status;
use jsonwebtoken::{jwk, DecodingKey, Validation};
use jsonwebtoken::jwk::AlgorithmParameters;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    NoAuthProvider,
    InvalidToken,
    InvalidTokenKID,
    InvalidTokenHeader,
    KeyNotRegisteredWithJWKS,
    InvalidTokenAlgorithm,
    InvalidRSAKey,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub preferred_username: String,
}

pub struct AuthProvider {
    jwks: Option<jwk::JwkSet>,
    jwks_provider: String,
}

impl AuthProvider {
    pub fn verify_token(&self, jwt: &str) -> Result<User, AuthError> {
        let jwks = match &self.jwks {
            Some(jwks) => jwks,
            None => return Err(AuthError::NoAuthProvider)
        };

        let header = match jsonwebtoken::decode_header(jwt) {
            Ok(header) => header,
            Err(_) => return Err(AuthError::InvalidTokenHeader),
        };

        let kid = match header.kid {
            Some(kid) => kid,
            None => return Err(AuthError::InvalidTokenKID),
        };

        let jwk = match jwks.find(&kid) {
            Some(jwk) => jwk,
            None => return Err(AuthError::KeyNotRegisteredWithJWKS),
        };

        return match &jwk.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let decoding_key = match DecodingKey::from_rsa_components(&rsa.n, &rsa.e) {
                    Ok(key) => key,
                    Err(_) => return Err(AuthError::InvalidRSAKey),
                };

                let algorithm = match jwk.common.algorithm {
                    Some(alg) => alg,
                    None => return Err(AuthError::InvalidTokenAlgorithm),
                };

                let mut validation = Validation::new(algorithm);
                validation.validate_exp = false;

                let decoded_token = match jsonwebtoken::decode::<User>(jwt, &decoding_key, &validation) {
                    Ok(token) => token,
                    Err(_) => return Err(AuthError::InvalidToken),
                };

                Ok(decoded_token.claims)
            },
            _ => Err(AuthError::InvalidTokenAlgorithm),
        }
    }

    pub async fn refresh_jwks(&mut self) {
        self.jwks = reqwest::get(self.jwks_provider.clone()).await.unwrap()
            .json().await.unwrap();
    }

    pub async fn new() -> AuthProvider {
        let jwks_provider = std::env::var("JWKS_PROVIDER").unwrap().to_string();

        let mut auth_provider = AuthProvider {
            jwks_provider,
            jwks: None
        };

        auth_provider.refresh_jwks().await;

        auth_provider
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_token = match req.headers().get_one("Authorization") {
            Some(token) => token,
            None => return Outcome::Failure((Status::Unauthorized, AuthError::MissingToken)),
        };

        let auth_provider = match req.rocket().state::<AuthProvider>() {
            Some(ap) => ap,
            None => return Outcome::Failure((Status::InternalServerError, AuthError::NoAuthProvider)),
        };

        return match auth_provider.verify_token(auth_token) {
            Ok(user) => Outcome::Success(user),
            Err(error) => Outcome::Failure((Status::Unauthorized, error)),
        };
    }
}
