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
