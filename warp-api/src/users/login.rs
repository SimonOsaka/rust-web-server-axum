use auth::encode_token;
use serde::Serialize;
use std::convert::Infallible;

pub async fn login() -> Result<impl warp::Reply, Infallible> {
    let token = encode_token(1);

    #[derive(Serialize)]
    struct User {
        name: String,
    }

    let u = User {
        name: "yy".to_string(),
    };

    Ok(warp::reply::with_header(
        warp::reply::json(&u),
        "token",
        token,
    ))
}
