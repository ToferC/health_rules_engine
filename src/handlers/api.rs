use actix_web::{web, get, post, HttpResponse, HttpRequest};
use crate::AppData;

#[post("/add_trip")]
pub async fn add_trip(
    _data: web::Data<AppData>,
    _req: HttpRequest,
) -> HttpResponse {
    let data= "Base String";
    
    HttpResponse::Ok().json(data)
}

#[get("/get_trips")]
pub async fn get_trips(
    _data: web::Data<AppData>,
    _req: HttpRequest,
) -> HttpResponse {
    HttpResponse::Ok().json("trips")
}

#[get("/get_trip_by_id/{trip_id}")]
pub async fn get_trip_by_id(
    _data: web::Data<AppData>, 
    _trip_id: web::Path<String>,
    _req: HttpRequest,
) -> HttpResponse {
    HttpResponse::Ok().json("trip")
}

#[post("/edit_trip/{trip_id}")]
pub async fn edit_trip(
    _data: web::Data<AppData>, 
    _trip_id: web::Path<String>,
    _req: HttpRequest,
) -> HttpResponse {
    HttpResponse::Ok().json("edit_trip")
}

#[post("/delete_trip/{trip_id}")]
pub async fn delete_trip(
    _data: web::Data<AppData>,
    _trip_id: web::Path<String>,
    _req: HttpRequest,
) -> HttpResponse {
    HttpResponse::Ok().json("confirmation")
}

