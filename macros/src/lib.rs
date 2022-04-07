mod api;

use api::router::expand_router;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn router(args: TokenStream, item: TokenStream) -> TokenStream {
    expand_router(args, item)
}
