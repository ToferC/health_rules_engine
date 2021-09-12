use actix_web::{web, get, post, HttpResponse, HttpRequest, Responder};
use crate::AppData;

#[post("/add_trip")]
pub async fn add_trip(
    data: web::Data<AppData>,
    req: HttpRequest,
) -> HttpResponse {
    let data= "Base String";
    
    HttpResponse::Ok().json(data)
}

#[get("/get_trips")]
pub async fn get_trips(
    data: web::Data<AppData>,
    req: HttpRequest,
) -> HttpResponse {
    HttpResponse::Ok().json("trips")
}

#[get("/get_trip_by_id/{trip_id}")]
pub async fn get_trip_by_id(
    data: web::Data<AppData>, 
    trip_id: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    HttpResponse::Ok().json("trip")
}

#[post("/edit_trip/{trip_id}")]
pub async fn edit_trip(
    data: web::Data<AppData>, 
    trip_id: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    HttpResponse::Ok().json("edit_trip")
}

#[post("/delete_trip/{trip_id}")]
pub async fn delete_trip(
    data: web::Data<AppData>,
    trip_id: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    HttpResponse::Ok().json("confirmation")
}

