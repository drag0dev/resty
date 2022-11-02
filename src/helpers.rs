use crate::types::Header;
use reqwest::header::{HeaderMap,HeaderValue};

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
