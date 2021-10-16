use chrono::{Duration, Utc};
use jsonwebtoken::errors::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use types::ID;

static TOKEN_SECRET: Lazy<String> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("jwt secret must set");
    secret
});
const TOKEN_PREFIX: &str = "Token ";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: ID,
    exp: u64, // seconds since the epoch
}

impl Claims {
    fn new(id: ID) -> Self {
        Self {
            sub: id,
            exp: (Utc::now() + Duration::days(30)).timestamp() as u64,
        }
    }

    pub fn id(&self) -> ID {
        self.sub
    }
}

pub fn encode_token(sub: ID) -> String {
    encode(
        &Header::default(),
        &Claims::new(sub),
        &EncodingKey::from_secret(TOKEN_SECRET.as_ref()),
    )
    .unwrap()
}

pub fn decode_token(token: &str) -> Result<Claims> {
    decode::<Claims>(
        token.trim_start_matches(TOKEN_PREFIX),
        &DecodingKey::from_secret(TOKEN_SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn encode_decode_token() {
        env::set_var("JWT_SECRET", "yes");

        let sub = 333;
        let token = encode_token(sub);
        println!("{}", token);
        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(TOKEN_SECRET.as_ref()),
            &Validation::default(),
        );
        if let Err(e) = &decoded {
            println!("decode err: {}", e);
        }

        println!("{}", decode_token(&token).unwrap().id());
        assert!(decoded.is_ok());
    }
}
