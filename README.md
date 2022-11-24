# <p align="center">resty</p>
<p align="center">A small framework for testing HTTP/WS web APIs I made for myself. It gets bothersome retesting endpoints every time you change something so you write a bunch of dumb tests to do it for you. </p>

- [HTTP tests example](tests/httptest.json)
- [WS tests example](tests/wstest.json)
#

## building/installing
```
cargo install --path ./
```
or
```
cargo build --release
```
and put the binary wherever you wish

## usage
```
resty [http*.json or ws*.json ...]
```
as many test files as you want  
test files starting with http are expected to have HTTP tests  
test files starting with ws are expected to have WS tests

## tests
format and fields, what they mean and their values

### HTTP
- **config**
    - **base_url** - URL to which requests are sent (**String**)
    - **port** - port to which requests are sent (**u16**) - optional
    - **pause** - wait time between tests in ms (**u32**) - optional
    - **timeout** - how long to wait for a response in ms (**u32**) - optional
    - **keep-session** - do you want to keep cookies/session between tests (**bool**)
- **tests** []
    - **request_end_point** - endpoint for this test (**String**)
    - **request_method** - request HTTP method (**String**)
    - **request_headers** - headers to be set on request - optional []
        - **header** - header name (**String**)
        - **value** - header value (**String**)
    - **request_params** - query parameters - optional []
        - **key** - query key (**String**)
        - **value** - query value (**String**)
    - **request_body** - request body (**String**) - optional
    - **response_code** - HTTP response code (**u16**)
    - **response_body** - response body (**String**) - optional
    - **response_headers** - expected headers on response - optional []
        - **header** - header name (**String**)
        - **value** - header value (**String**)
    
### WS