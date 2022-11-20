use actix_web::{Responder, get, post, web, HttpResponse, HttpRequest, Error};
use crate::types::Reply;
use actix_web_actors::ws;
use crate::ws::WsMirror;

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

#[get("/mirror_headers")]
async fn mirror_headers(req: HttpRequest) -> impl Responder{
    let header_map = req.headers();
    let mut response = HttpResponse::Ok();
    for header in header_map.keys(){
        response.insert_header((header, header_map.get(header).unwrap()));
    }
    response
}

#[get("/ws_mirror")]
async fn ws_mirror(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error>{
    let resp = ws::start(WsMirror{}, &req, stream);
    resp
}

#[get("/params")]
async fn params(req: HttpRequest) -> Result<HttpResponse, Error>{
    let res = String::from(req.query_string());
    Ok(HttpResponse::Ok().body(res))
}
