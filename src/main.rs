use std::{env, fs, process::exit};
use colored::Colorize;

mod http_config;
mod http_client;
mod helpers;
mod ws_config;
mod ws_client;
mod execute;

#[tokio::main]
async fn main() {
    // load file names
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}: missing file name",  "error".red().bold());
        exit(1);
    }

    for test_config in args.iter().skip(1){
        println!("Running {}", test_config);
        println!("----------------------------------------");

        // check if the file/path exists
        let file_contents = fs::read_to_string(test_config);
        if file_contents.is_err(){
            println!("{}: opening file: {}",  "error".red().bold(), file_contents.err().unwrap());
            exit(1);
        }
        let file_contents = file_contents.unwrap();

        let success;
        let failed;

        // determine type of tests
        if args[1].starts_with("ws"){
            let master_struct: Result<ws_config::MasterStruct, serde_json::Error> = serde_json::from_str(&file_contents);
            if master_struct.is_err(){
                println!("{}: parsing ws test file: {}",  "error".red().bold(), master_struct.err().unwrap());
                exit(1);
            }
            let master_struct = master_struct.unwrap();
            (success, failed) = execute::ws(master_struct).await;

        }else if args[1].starts_with("http"){ // covers both http and https
            let master_struct: Result<http_config::MasterStruct, serde_json::Error> = serde_json::from_str(&file_contents);
            if master_struct.is_err(){
                println!("{}: parsing http test file: {}",  "error".red().bold(), master_struct.err().unwrap());
                exit(1);
            }
            let master_struct = master_struct.unwrap();
            (success, failed) = execute::http(master_struct).await;

        }else {
            panic!("{}: missing or wrong schema in the test file name \"{}\"!",
                "error".red().bold(), args[1]);
        };

        println!("\nResults ({}) -> {}: {} {}: {}",
            test_config,
            "success".green().bold(),
            success,
            "failed".red().bold(),
            failed);
        println!("----------------------------------------\n");
    }
}
