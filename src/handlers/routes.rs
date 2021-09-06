use actix_web::web;

use crate::handlers::{
    index,
    api_base,
    playground_handler,
};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(api_base);
    config.service(playground_handler);
}
