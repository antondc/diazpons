#![allow(
    clippy::new_ret_no_self,
    clippy::let_and_return,
    clippy::large_enum_variant,
    clippy::too_many_arguments,
    clippy::module_inception,
    clippy::redundant_pattern_matching,
    clippy::needless_collect,
    unused_imports
)]

use dotenvy::dotenv;
mod infrastructure;
use infrastructure::http::run_http;
mod domain;
mod presentation;
mod application;

fn main() {
    dotenv().ok();

    run_http();
}
