mod base;
mod routes;
mod api;
mod endpoints;

pub use self::routes::init_routes;

pub use self::base::{index, api_base};
pub use self::api::{get_trip_by_id, get_trips, add_trip, edit_trip, delete_trip};
pub use self::endpoints::{playground_handler};