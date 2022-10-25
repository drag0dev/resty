// for each test in test.json for the framework a separate handler is written

use actix_web::*;

#[get("/get_basic_test")]
async fn get_basic_test() -> impl Responder{
    HttpResponse::Ok().body("Hello world!")
}
