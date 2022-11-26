use crate::http_config::Header;
use reqwest::header::HeaderMap;
use colored::Colorize;
use crate::ws_config::MessageType;
use tokio_tungstenite::tungstenite::{
    Message,
    protocol::frame::coding::CloseCode,
};

use textwrap::wrap;

pub fn header_match(header: &Header, result_headers: &HeaderMap) -> bool{
    if let Some(res_header) = result_headers.get(&header.header){
        let res_header_value = res_header.to_str();
        if res_header_value.is_err(){
            return false;
        }
        return res_header_value.unwrap() == header.value;
    }
    false
}

pub fn body_match(body_one: &String, body_two: &String, index: usize) -> bool{
    if body_one != body_two{
        let mut start = misamatch_slice(body_one, body_two);
        println!("{} ({}) - body not matching starting: {}", "fail".red().bold(), index+1, start);
        if body_one.len() > 0{
            let expected = wrap(body_one, 75);
            let mut color = false;
            println!("\tExpected:");
            for line in expected{
                if !color && start < line.len(){
                    println!("\t\t{}{}",
                        &line[0..start],
                        &line[start..].red(),
                        );
                    color = true;
                    continue;
                }
                if color{
                    println!("\t\t{}", line.red());
                }else{
                    println!("\t\t{}", line);
                    start -= line.len();
                }
            }

            /*println!("\tExpected: {}{}",
                &expected[0..start],
                &expected[start..].red()
            );*/

            println!("\tGot: \n\t\t{}\n",
                body_two
            );
        }
        return true;
    }
    false
}

/// find where the difference between s1 and s2 starts
pub fn misamatch_slice<'a>(s1: &String, s2: &'a String) -> usize{
    let mut start = 0;

    let mut s1_iter = s1.chars();
    let mut s2_iter = s2.chars();
    let mut s1c: Option<char>;
    let mut s2c: Option<char>;
    loop{
        s1c = s1_iter.next();
        s2c = s2_iter.next();
        if s1c.is_some() && s2c.is_some(){
            if s1c.unwrap() != s2c.unwrap(){
                break;
            }
        }else{
            break;
        }

        start += 1;
    }
    start
}

/// get type of the return message
pub fn message_type(msg: &Message) -> MessageType{
    if msg.is_text() {return MessageType::Text;}
    if msg.is_ping() {return MessageType::Ping;}
    if msg.is_pong() {return MessageType::Pong;}
    if msg.is_binary() {return MessageType::Binary;}
    MessageType::Close
}

/// CloseCode from String
pub fn close_code_from_str(code: &String) -> Option<CloseCode>{
    match &code.to_uppercase()[..]{
        "NORMAL" => {Some(CloseCode::Normal)},
        "AWAY" => {Some(CloseCode::Away)},
        "PROTOCOL" => {Some(CloseCode::Protocol)},
        "UNSUPPORTED" => {Some(CloseCode::Unsupported)},
        "STATUS" => {Some(CloseCode::Status)},
        "ABNORMAL" => {Some(CloseCode::Abnormal)},
        "INVALID" => {Some(CloseCode::Invalid)},
        "POLICY" => {Some(CloseCode::Policy)},
        "SIZE" => {Some(CloseCode::Size)},
        "ERROR" => {Some(CloseCode::Error)},
        "EXTENSION" => {Some(CloseCode::Extension)},
        "RESTART" => {Some(CloseCode::Restart)},
        "AGAIN" => {Some(CloseCode::Again)},
        _ => None,
    }
}
