use actix_web::web;

use crate::handlers::{
    index,
    api_base,
    playground_handler,
    
    // API
    get_trips,
    get_trip_by_id,
    edit_trip,
    delete_trip,
    add_trip,
};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(api_base);
    config.service(playground_handler);
}
