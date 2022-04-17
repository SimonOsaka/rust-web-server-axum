mod api;
mod args;
mod repository;
mod util;

use api::{from_error::expand_from_error, router::expand_router};
use proc_macro::TokenStream;
use repository::from_model::expand_from_model;

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

/// Database one model will generate `insert`, `update`, `delete`, `get` method.
/// Two combine model will generate `find` method. `**Fields` for `get` and `find` method to add conditions.
///
/// # Macro
/// - struct `#[derive(FromModel)]`
/// - struct `#[from_model(table_name = "...")]`
/// - field primary_key `#[from_model(primary_key)]`
/// - field foreign_key `#[from_model(table_name = "...", model = "...")]`
///
/// Note:
/// - if primary_key not set, only insert method will be generated.
/// - foreign_key depend primary_key.
///
/// # Examples
/// ```rust
/// #[derive(FromModel)]
/// #[from_model(table_name = "test_car")]
/// struct TestCar {
///     #[from_model(primary_key)]
///     id: i64,
///     name: String,
///     #[from_model(table_name = "test_user", model = "TestUser", primary_key = "id")]
///     user_id: i64,
/// }
///
/// // One model
/// #[derive(FromModel)]
/// #[from_model(table_name = "test_user")]
/// struct TestUser {
///     #[from_model(primary_key)]
///     id: i64,
///     name: String,
/// }
///
/// // generated code
/// const TESTCAR_SINGLE_FIELDS: &[&str; 3] = &[
///     "test_car.id",
///     "test_car.name",
///     "test_car.user_id",
/// ];
/// const TESTCAR_MULTI_FIELDS: &[&str; #multi_size] = &[
///     "(",
///     "test_car.id",
///     "test_car.name",
///     "test_car.user_id",       
///     ") AS \"test_car\"",
/// ];
/// impl TestCar {
///     fn insert(&self) -> ... {
///         ...
///     }
///     fn update(&self) -> ... {
///         ...
///     }
///     fn delete(&self) -> ... {
///         ...
///     }
///     fn get(fields: Vec<XxxFields>, ...) -> ... {
///         ...
///     }
/// }
///
/// // Two combine model
/// struct TestCarTestUser {
///     test_car: TestCar,
///     test_user: TestUser,
/// }
/// impl TestCarTestUser {
///     fn find(test_car_fields: Vec<TestCarFields>, test_user_fields: Vec<TestUserFields>, ...) -> Result<Vec<(TestCar, TestUser)>, ...> {
///         ...
///     }
/// }
/// // TestUser same as TestCar
/// ```
#[proc_macro_derive(FromModel, attributes(from_model))]
pub fn from_model_derive(input: TokenStream) -> TokenStream {
    expand_from_model(input)
}
