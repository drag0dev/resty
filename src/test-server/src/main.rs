use actix_web::{
    HttpServer,
    App,
    middleware::Logger,
};
use colored::Colorize;
use env_logger::Env;

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

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("[%t-%D]%a %s UA:%{User-Agent}i CT:%{Content-Type}i"))
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
