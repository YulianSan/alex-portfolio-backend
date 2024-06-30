use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};

use crate::domain::{entities::claims::Claims, usecases::auth_service::AuthService};

struct AuthServiceJwt {}

impl AuthService for AuthServiceJwt {
    fn new() -> Self {
        Self {}
    }

    fn generate_token(&self, user_uuid: &str) -> String {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
            + 3600usize;

        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref());

        let claims = Claims {
            sub: user_uuid.to_string(),
            exp,
        };

        encode(&header, &claims, &encoding_key).unwrap()
    }

    fn validate_token(&self, token: &str) -> bool {
        let decoding_key = DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref());
        decode::<Claims>(&token, &decoding_key, &jsonwebtoken::Validation::default()).is_ok()
    }
}
