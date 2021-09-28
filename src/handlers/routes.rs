use actix_web::web;

use crate::handlers::{
    index,
    api_base,
    playground_handler,
    graphql,
    
    // API
    // get_trips,
    // get_trip_by_id,
    // edit_trip,
    // delete_trip,
    // add_trip,
};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(api_base);
    config.route("/graphql", web::post().to(graphql));
    config.route("/graphql", web::get().to(playground_handler));
}
