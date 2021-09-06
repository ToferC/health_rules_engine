mod base;
mod routes;

pub use self::routes::init_routes;

pub use self::base::{index, api_base, playground_handler};