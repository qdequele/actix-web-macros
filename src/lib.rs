#![recursion_limit = "512"]
//! Actix-web codegen module
//!
//! Generators for routes and scopes
//!
//! ## Route
//!
//! Macros:
//!
//! - [get](attr.get.html)
//! - [post](attr.post.html)
//! - [put](attr.put.html)
//! - [delete](attr.delete.html)
//! - [head](attr.head.html)
//! - [connect](attr.connect.html)
//! - [options](attr.options.html)
//! - [trace](attr.trace.html)
//! - [patch](attr.patch.html)
//!
//! ### Attributes:
//!
//! - `"path"` - Raw literal string with path for which to register handle. Mandatory.
//! - `guard="function_name"` - Registers function as guard using `actix_web::guard::fn_guard`
//! - `wrap=Middleware` - Registers a ressource middleware.
//!
//! ## Notes
//!
//! Function name can be specified as any expression that is going to be accessible to the generate
//! code (e.g `my_guard` or `my_module::my_guard`)
//!
//! ## Example:
//!
//! ```rust
//! use actix_web::HttpResponse;
//! use actix_web_codegen::get;
//! use futures::{future, Future};
//!
//! #[get("/test")]
//! async fn async_test() -> Result<HttpResponse, actix_web::Error> {
//!     Ok(HttpResponse::Ok().finish())
//! }
//! ```

extern crate proc_macro;

mod route;

use proc_macro::TokenStream;

/// Creates route handler with `GET` method guard.
///
/// Syntax: `#[get("path"[, attributes])]`
///
/// ## Attributes:
///
/// - `"path"` - Raw literal string with path for which to register handler. Mandatory.
/// - `guard="function_name"` - Registers function as guard using `actix_web::guard::fn_guard`
/// - `wrap=Middleware` - Registers a ressource middleware.
#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    route::generate(args, input, route::GuardType::Get)
}

/// Creates route handler with `POST` method guard.
///
/// Syntax: `#[post("path"[, attributes])]`
///
/// Attributes are the same as in [get](attr.get.html)
#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    route::generate(args, input, route::GuardType::Post)
}

/// Creates route handler with `PUT` method guard.
///
/// Syntax: `#[put("path"[, attributes])]`
///
/// Attributes are the same as in [get](attr.get.html)
#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    route::generate(args, input, route::GuardType::Put)
}

/// Creates route handler with `DELETE` method guard.
///
/// Syntax: `#[delete("path"[, attributes])]`
///
/// Attributes are the same as in [get](attr.get.html)
#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    route::generate(args, input, route::GuardType::Delete)
}

/// Creates route handler with `HEAD` method guard.
///
/// Syntax: `#[head("path"[, attributes])]`
///
/// Attributes are the same as in [head](attr.head.html)
#[proc_macro_attribute]
pub fn head(args: TokenStream, input: TokenStream) -> TokenStream {
    route::generate(args, input, route::GuardType::Head)
}

/// Creates route handler with `CONNECT` method guard.
///
/// Syntax: `#[connect("path"[, attributes])]`
///
/// Attributes are the same as in [connect](attr.connect.html)
#[proc_macro_attribute]
pub fn connect(args: TokenStream, input: TokenStream) -> TokenStream {
    route::generate(args, input, route::GuardType::Connect)
}

/// Creates route handler with `OPTIONS` method guard.
///
/// Syntax: `#[options("path"[, attributes])]`
///
/// Attributes are the same as in [options](attr.options.html)
#[proc_macro_attribute]
pub fn options(args: TokenStream, input: TokenStream) -> TokenStream {
    route::generate(args, input, route::GuardType::Options)
}

/// Creates route handler with `TRACE` method guard.
///
/// Syntax: `#[trace("path"[, attributes])]`
///
/// Attributes are the same as in [trace](attr.trace.html)
#[proc_macro_attribute]
pub fn trace(args: TokenStream, input: TokenStream) -> TokenStream {
    route::generate(args, input, route::GuardType::Trace)
}

/// Creates route handler with `PATCH` method guard.
///
/// Syntax: `#[patch("path"[, attributes])]`
///
/// Attributes are the same as in [patch](attr.patch.html)
#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    route::generate(args, input, route::GuardType::Patch)
}
