use actix_web::{HttpServer, App};
use colored::Colorize;

mod tests;

#[actix_web::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port = match args.get(1){
        Some(p) => {
            let p_parse = p.parse::<u16>();
            if p_parse.is_err(){
                panic!("{}: parsing port \"{}\"", "error".red(), p);
            }
            p_parse.unwrap()
        },
        None => {8080}
    };

    let server = HttpServer::new(|| {
        App::new()
            .service(tests::get_basic_test)
    }).bind(("127.0.0.1", port));

    if server.is_err(){
        panic!("{}: binding to port \"{}\": {}", "error".red(), port, server.err().unwrap());
    }
    println!("{} on port {}", "running".green().bold(), port);
    let server = server.unwrap();
    server.run().await;
}
