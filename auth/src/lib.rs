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
    exp: u64,           // seconds since the epoch
    name: String,       // username
    roles: Vec<String>, // user role [user, view]
}

impl Claims {
    fn new(id: ID, name: String, roles: Vec<String>) -> Self {
        Self {
            sub: id,
            exp: (Utc::now() + Duration::days(30)).timestamp() as u64,
            name,
            roles,
        }
    }

    pub fn get_id(&self) -> ID {
        self.sub
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
}

pub fn encode_token(sub: ID, name: String, roles: Vec<String>) -> String {
    encode(
        &Header::default(),
        &Claims::new(sub, name, roles),
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

pub fn role_view() -> Claims {
    Claims {
        sub: i64::MAX,
        exp: u64::MAX,
        name: "viewer".to_string(),
        roles: vec!["view".into()],
    }
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum JWTError {
    #[error("Missing authorization key")]
    Missing,
    #[error("Authorization wrong")]
    Invalid,
}

impl warp::reject::Reject for JWTError {}
