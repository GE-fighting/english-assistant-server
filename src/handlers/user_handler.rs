// This file will contain user-related handlers
use actix_web::{HttpResponse, Responder};


pub async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, welcome to the English Learning Assistant!")
}