mod api;

use api::{from_error::expand_from_error, router::expand_router};
use proc_macro::TokenStream;

/// Api function macro
/// - generate Router
/// - merge format `<method> + '_' + <function_name>()`
///
/// # Examples
/// ```rust
/// #[router(path = "/api/test", method = "get")]
/// pub async fn test(..){}
///
/// Router::new().merge(get_test());
/// ```
#[proc_macro_attribute]
pub fn router(args: TokenStream, item: TokenStream) -> TokenStream {
    expand_router(args, item)
}

/// Api error convert
/// - generate impl `From`, `Error`, `Display`
///
/// # Examples
/// ```rust
/// #[FromError]
/// enum PlayError {
///     #[from_error(code = "play-nocard-error", status = "forbidden")]
///     Nocard,
///     #[from_error(code = "play-card-error", status = "forbidden")]
///     Card { id: i64, name: String },
///     #[from_error(code = "play-can-not-error", status = "forbidden")]
///     CanNotPlay(Error)
/// }
/// ```
#[proc_macro_derive(FromError, attributes(from_error))]
pub fn from_error_derive(input: TokenStream) -> TokenStream {
    expand_from_error(input)
}
