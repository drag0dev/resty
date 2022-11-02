use actix_web::{Responder, get, post, web, HttpResponse};
use crate::types::Reply;

// for each test in test.json for the framework a separate handler is written
#[get("/get_basic_test")]
async fn get_basic_test() -> impl Responder{
    HttpResponse::Ok().body("Hello world!")
}

#[post("/json_mirror")]
async fn json_mirror(info: web::Json<Reply>) -> impl Responder{
    let rep = Reply{message: String::from(&info.message)};
    HttpResponse::Ok().json(rep)
}
