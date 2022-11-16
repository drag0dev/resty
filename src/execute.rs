use colored::Colorize;
use std::time::Duration;
use crate::{
    http_config::MasterStruct,
    http_client::Client,
    helpers::*,
    ws_config::MessageType,
    ws_client,
    ws_config,
};

pub async fn http(master_struct: MasterStruct) -> (u32, u32){
    let master_client = Client::new(&master_struct.config);
    if master_client.is_err(){
        println!("{} setting up http client: {}", "error".red().bold(), master_client.err().unwrap());
        std::process::exit(1);
    }

    let mut success = 0;
    let mut failed = 0;
    let timeout = if master_struct.config.timeout.is_some(){
        master_struct.config.timeout.unwrap()
    }else{
        0
    };

    let master_client = master_client.unwrap();
    for (i, t) in master_struct.tests.iter().enumerate(){
        let result = master_client.exec_test(t).await;
        if result.is_err(){
            println!("{}: executing a test {}", "error".red().bold(), i+1);
            failed += 1;
        }else{
            let result = result.unwrap();
            let mut failed_check: bool = false;

            // check status code
            if t.response_code != result.status().as_u16(){
                println!("{} ({}) - response code not matching {} != {}",
                "fail".red().bold(), i+1, t.response_code, result.status().as_u16());
                failed_check = true;
            }

            // check headers if required
            if let Some(test_headers) = &t.response_headers{
                let mut first: bool = false;
                let res_headers = result.headers();
                for (header_i, h) in test_headers.iter().enumerate(){
                    if !header_match(h,  res_headers){
                        if !first{
                            println!("{} ({}) - headers not matching:", "fail".red().bold(), i+1);
                            failed_check = true;
                            first = true;
                        }
                        println!("\t({}) {} not matching ", header_i+1, h.header);
                        if let Some(value) = res_headers.get(&h.header){
                            println!("\t\tTest header value: {}", h.value);
                            println!("\t\tResult header value: {}", value.to_str().unwrap());
                        }else{
                            println!("\t\t  missing header");
                        }
                    }
                }
            }

            // check body if required
            if let Some(body) = &t.response_body{
                let res_body = result.bytes().await;
                if res_body.is_err(){
                    println!("{} ({}) - error getting response body", "fail".red().bold(), i+1);
                    failed_check = true;
                }else{
                    let res_body = res_body.unwrap();
                    let res_body_str = res_body.iter().map(|b| *b as char).collect::<String>();
                    if body_match(body, &res_body_str, i){
                        failed_check = true;
                    }
                }
            }

            if failed_check{
                failed += 1;
            }else{
                success += 1;
                println!("{} ({}) - /{}", "success".green().bold(), i+1, t.request_end_point);
            }
        }

        if timeout > 0{
            std::thread::sleep(Duration::from_millis(timeout as u64));
        }
    }
    (success, failed)
}

pub async fn ws(master_struct: ws_config::MasterStruct) -> (u32, u32){
    let master_client = ws_client::ClientWS::new(&master_struct.config).await;
    if master_client.is_err(){
        println!("{}: making a ws client: {}",
            "error".bold().red(), master_client.err().unwrap());
        println!("skipping this file...");
        return (0, 0);
    }
    let mut master_client = master_client.unwrap();
    let mut success = 0;
    let mut failed = 0;
    let timeout = if master_struct.config.timeout.is_some(){
        master_struct.config.timeout.unwrap()
    }else{
        0
    };

    for (i, t) in master_struct.tests.iter().enumerate(){
        if (t.response_type.is_none() && t.response_data.is_some()) ||
           (t.response_type.is_some() && t.response_data.is_none()){
            println!("{} ({}): response_type and response_data do not align, skipping test", "error".bold().red(), i+1);
            failed += 1;
            continue;
        }

        // execute test
        let result =
            if t.send_type == MessageType::Text{
                let payload = if t.send_data.is_some(){
                    t.send_data.as_ref().unwrap().clone()
                }else{
                    String::from("")
                };
                master_client.text(payload).await
            }else if t.send_type == MessageType::Close{
                todo!();
            }else {
                // these type of messages accept Vec<u8> and not a String
                let payload = if t.send_data.is_some(){
                    t.send_data.as_ref().unwrap().as_bytes().to_vec()
                }else{
                    Vec::new()
                };
                match t.send_type{
                    MessageType::Binary => master_client.binary(payload).await,
                    MessageType::Ping => master_client.ping(payload).await,
                    MessageType::Pong => master_client.pong(payload).await,
                    _ => unreachable!(),
                }
            };

        // check result of the test
        let mut fail_check = false;
        if result.is_err(){
            println!("{}: executing a test{}", "error".red().bold(), i+1);
            fail_check = true;
        }else{
            let result = result.unwrap();

            // if we get a body and expect a body
            if t.response_type.is_some() && result.is_some(){
                let result = result.unwrap();
                if result.is_err(){
                    println!("\t{} - reading received body", "error".red().bold());
                    fail_check = true;
                }else{
                    let result = result.unwrap();

                    // check type of message
                    let result_type = message_type(&result);
                    let test_type = t.response_type.as_ref().unwrap();
                    if test_type != &result_type{
                        println!("\t{}: message types not matching {} != {}", "fail".red().bold(), test_type, result_type);
                        fail_check = true;
                    }

                    // check bodies
                    let result_body = result.into_data().iter().map(|b| *b as char).collect::<String>();
                    let expected_body = t.response_data.as_ref().unwrap();
                    if body_match(expected_body, &result_body, i){
                        fail_check = true;
                    }
                }

            // if we expect a body but dont recieve any
            }else if t.response_type.is_some() && result.is_none(){
                println!("{} ({}) - result missing body",
                    "error".bold().red(), i+1
                );
                fail_check = true;
            // if we don't expect a body but recieve one
            }else if t.response_type.is_none() && result.is_some(){
                println!("{} ({}) - result has a body",
                    "error".bold().red(), i+1
                );
                let result = result.as_ref().unwrap();
                if result.is_err(){
                    println!("\t{} - reading received body", "error".red().bold());
                }else{
                    println!("\t{}", result.as_ref().unwrap());
                }
                fail_check = true;
            }
        }

        if fail_check{
            failed += 1;
        }else{
            success += 1;
            println!("{} ({}) - {}", "success".green().bold(), i+1, t.send_type);
        }

        // timeout if set
        if timeout > 0{
            std::thread::sleep(Duration::from_millis(timeout as u64));
        }
    }
    (success, failed)
}
