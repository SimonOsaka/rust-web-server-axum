use crate::http::error::Error;
use gloo::storage::{LocalStorage, Storage};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};

#[cfg(not(feature = "mock"))]
use crate::http::error::ErrorInfo;
#[cfg(not(feature = "mock"))]
use dotenv_codegen::dotenv;
#[cfg(not(feature = "mock"))]
const API_ROOT: &str = dotenv!("API_ROOT");

#[cfg(feature = "mock")]
static MOCK_FILES: once_cell::sync::OnceCell<Vec<String>> = once_cell::sync::OnceCell::new();

pub fn mock_init(mocks: Vec<String>) {
    println!("num of mocks {}", mocks.len());
    #[cfg(feature = "mock")]
    MOCK_FILES.set(mocks).expect("Mock files set required");
}

#[cfg(feature = "mock")]
pub async fn request<B, T>(method: reqwest::Method, url: String, _body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let mock_response_str = mock(url, method.as_str().to_lowercase());
    let mock_response_json: T = serde_json::from_str(&mock_response_str).unwrap();
    Ok(mock_response_json)
}

#[cfg(not(feature = "mock"))]
pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", API_ROOT, url);
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");
    if let Some(token) = get_token() {
        builder = builder.bearer_auth(token);
    }

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                // log::debug!("Response: {:?}", data);
                Ok(data)
            } else {
                Err(Error::DeserializeError)
            }
        } else {
            match data.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => {
                    let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}

/// Delete request
#[allow(dead_code)]
pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, url, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::POST, url, body).await
}

/// Put request with a body
#[allow(dead_code)]
pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::PUT, url, body).await
}

#[cfg(feature = "mock")]
pub(crate) fn mock(url: String, method: String) -> String {
    let mock_json_str_includes = MOCK_FILES.get().unwrap();
    // let mock_json_str_includes = vec![
    //     include_str!("../../mock/auth.json"),
    //     include_str!("../../mock/list.json"),
    //     include_str!("../../mock/menu.json"),
    //     include_str!("../../mock/site.json"),
    // ];
    for mock_json_str_include in mock_json_str_includes {
        let mock_json_value: serde_json::Value =
            serde_json::from_str(mock_json_str_include).unwrap();
        if mock_json_value.is_array() {
            let mock_array = mock_json_value.as_array();
            if let Some(mock_array) = mock_array {
                for mock in mock_array {
                    let url = url.clone();
                    let path_url = if url.contains('?') {
                        let split_url: Vec<&str> = url.split('?').collect();
                        split_url
                            .first()
                            .unwrap_or_else(|| panic!("url ? failed"))
                            .to_string()
                    } else {
                        url
                    };

                    let mock_url = mock["url"].as_str().unwrap().to_string();
                    let mock_method = mock["method"].as_str().unwrap().to_string();

                    if mock_url == path_url && mock_method == method {
                        // println!("{mock_path_url} {url} {mock_method} {method}");
                        let mock_data = mock["data"].as_object();
                        // println!("{:?}", mock_data);
                        return serde_json::to_string(mock_data.unwrap()).unwrap();
                    }
                }
            }
        }
    }
    panic!("no mock json data, url={}, method={}", &url, &method);
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

const TOKEN_KEY: &str = "rust-web-server-yew::token";

static TOKEN: Lazy<RwLock<Option<String>>> = Lazy::new(|| {
    if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
        RwLock::new(Some(token))
    } else {
        RwLock::new(None)
    }
});

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    if let Some(t) = token.clone() {
        LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

#[cfg(feature = "mock")]
#[test]
fn test_mock_user() {
    let str = mock(
        "/users".to_string(),
        reqwest::Method::POST.as_str().to_lowercase(),
    );
    println!("{str}");
    let json: crate::types::EmptyWrapper = serde_json::from_str(&str).unwrap();
    println!("{json:?}");
}

#[cfg(feature = "mock")]
#[test]
fn test_mock_list() {
    let str = mock(
        "/list?page=1".to_string(),
        reqwest::Method::GET.as_str().to_lowercase(),
    );
    println!("{str}");
}

#[cfg(feature = "mock")]
#[test]
fn test_mock_menu() {
    let str = mock(
        "/menu".to_string(),
        reqwest::Method::GET.as_str().to_lowercase(),
    );
    println!("{str}");
}
