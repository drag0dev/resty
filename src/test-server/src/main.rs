use actix_web::{HttpServer, App};
use colored::Colorize;

mod types;
mod tests;

#[actix_web::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port = match args.get(1){
        Some(p) => {
            let p = p.parse::<u16>();
            if p.is_err(){
                panic!("{}: parsing port \"{}\"", "error".red(), p.unwrap());
            }
            p.unwrap()
        },
        None => {8080}
    };

    let server = HttpServer::new(|| {
        App::new()
            .service(tests::get_basic_test)
            .service(tests::json_mirror)
    }).bind(("127.0.0.1", port));

    if server.is_err(){
        panic!("{}: binding to port \"{}\": {}", "error".red(), port, server.err().unwrap());
    }
    println!("{} on port {}", "running".green().bold(), port);
    let server = server.unwrap();
    server.run().await;
}
