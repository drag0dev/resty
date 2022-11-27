use std::{
    env,
    fs,
    process::exit,
    path::Path,
};
use colored::Colorize;

mod http_config;
mod http_client;
mod helpers;
mod ws_config;
mod ws_client;
mod execute;

fn get_files_from_dir(path_str: &str) -> Vec<String>{
    let mut files = Vec::new();
    let path = Path::new(path_str);
    let entries = path.read_dir();
    if entries.is_err(){
        println!("{}: reading files in the diretory ({}): {}",
        "error".bold().red(),
        path_str,
        entries.err().unwrap());
        exit(1);
    }

    for file in entries.unwrap(){
        if file.is_ok(){
            let file = file.unwrap();
            let file_path = file.path();
            if file_path.is_file(){
                let file_full_path = file_path.to_str();
                if file_full_path.is_none(){
                    println!("{}: getting file path", "error".bold().red());
                    exit(1);
                }
                let file_name = file_path.file_name().unwrap(); // unwrap because its already been checked that its a file
                let file_name = file_name.to_str();
                if file_name.is_none(){
                    println!("{}: getting file name", "error".bold().red());
                    exit(1);
                }
                let file_name = file_name.unwrap();
                if file_name.starts_with("http") || file_name.starts_with("ws"){
                    files.push(file_full_path.unwrap().to_string());
                }
            }
        }
    }
    files
}

#[tokio::main]
async fn main() {
    // load file names
    let args: Vec<String> = env::args().collect();
    let files;
    let iter;
    let len;

    // if no arguments take current working directory
    // skip(0) required to align iter type in all branches
    if args.len() == 1 {
        let dir = std::env::current_dir();
        if dir.is_err(){
            println!("{}: getting current directory: {}", "error".bold().red(), dir.err().unwrap());
            exit(1);
        }

        let dir = dir.unwrap();
        let dir_str = dir.to_str();
        if dir_str.is_none(){
            println!("{}: malformed current directory", "error".bold().red());
            exit(1);
        }

        files = get_files_from_dir(dir_str.unwrap());
        iter = files.iter().skip(0);
        len = files.len();
    }

    // directory or a single file
    else if args.len() == 2 {
        let path = Path::new(&args[1]);
        if path.is_file(){
            files = vec![args[1].clone()];
            iter = files.iter().skip(0);
            len = 1;
        }else{
            files = get_files_from_dir(&args[1]);
            iter = files.iter().skip(0);
            len = files.len();
        }
    }

    // multiple test files
    else {
        iter = args.iter().skip(1);
        len = args.len() - 1;
    }

    if len == 0 {
        println!("No test files, exiting...");
    }

    for test_config in iter{
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
        let file_name = Path::new(test_config).file_name().unwrap().to_str().unwrap(); // will always be something because a file has already been read

        // determine type of tests
        if file_name.starts_with("ws"){
            let master_struct: Result<ws_config::MasterStruct, serde_json::Error> = serde_json::from_str(&file_contents);
            if master_struct.is_err(){
                println!("{}: parsing ws test file: {}",  "error".red().bold(), master_struct.err().unwrap());
                exit(1);
            }
            let master_struct = master_struct.unwrap();
            (success, failed) = execute::ws(master_struct).await;

        }else if file_name.starts_with("http"){
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
        }

        println!("\nResults ({}) -> {}: {} {}: {}",
            test_config,
            "success".green().bold(),
            success,
            "failed".red().bold(),
            failed);
        println!("----------------------------------------\n");
    }
}
