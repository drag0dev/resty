use std::{env, fs, process::exit};
use colored::Colorize;

mod types;
mod client;
use types::*;
use client::Client;


fn main() {
    // load file name
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}: missing file name",  "error".red());
        exit(1);
    }
    let test_config: &String = &args[1];

    // check if the file/path exists
    let file_contents = fs::read_to_string(test_config);
    if file_contents.is_err(){
        println!("{}: opening file: {}",  "error".red(), file_contents.err().unwrap());
        exit(1);
    }
    let file_contents = file_contents.unwrap();

    // parsing config file into json
    let master_struct: Result<MasterStruct, serde_json::Error> = serde_json::from_str(&file_contents);
    if master_struct.is_err(){
        println!("{}: parsing config file: {}",  "error".red(), master_struct.err().unwrap());
        exit(1);
    }
    let master_struct = master_struct.unwrap();
    let master_client = Client::new(master_struct.config);
}
