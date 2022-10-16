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

pub fn slice<'a>(test_header: &'a Header, res_header: &HeaderValue) -> (&'a str, usize){
    let mut start = 0;
    let mut end;

    // find start of the diff
    for (i, (t, r)) in test_header.value.chars()
        .zip(res_header.to_str().unwrap().chars())
        .enumerate(){
            if t != r{
                start = i;
                break;
            }
    }

    end = start + 15;
    if end >= test_header.value.len(){
        end = test_header.value.len()-1;
    }
    let diff_char = start;
    if start >= 15  {
        start -= 15;
    }

    (&test_header.value[start..end], diff_char)
}
