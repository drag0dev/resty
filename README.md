# <p align="center">**resty**</p>
<p align="center">resty if a framework for testing HTTP/WS web APIs I made for myself. It gets bothersome retesting endpoints every time you change something so you write a bunch of dumb tests to do it for you.</p>

- [HTTP tests example](tests/httptest.json)
- [WS tests example](tests/wstest.json)
#

## Building
```
cargo install --path ./
```
or
```
cargo build --release
```
and put the binary wherever you wish

## Usage
if no  arguments is provided tests in the current working directory are going to be executed
```
resty
```

if directory is provided all the tests in that directory are going to be executed
```
resty dir
```

or just pass test files
```
resty [http*.json or ws*.json ...]
```
as many test files as you want  
test files starting with http are expected to have HTTP tests  
test files starting with ws are expected to have WS tests

## Tests
format and fields, what they mean and their values
### HTTP
- **config**
    - **base_url** - URL to which requests are sent (**String**)
    - **port** - port to which requests are sent (**u16**) - optional
    - **pause** - wait time between tests in ms (**u32**) - optional
    - **timeout** - how long to wait for a response in ms (**u32**) - optional (default 5s)
    - **keep-session** - do you want to keep cookies/session between tests (**bool**)
    - **suppress_extra_headers** - if extra headers are not suppressed they will be printed but will not fail test (**bool**) - optional (default true)
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
- **config**
    - **url** - URL to which frames are sent (**String**)
    - **port** - port to which frames are sent (**u16**) - optional
    - **pause** - wait time between tests in ms (**u32**) - optional
    - **timeout** - how long to wait for a response in ms (**u32**) - optional (default 5s)
- **tests** []
    - **send_type** - type of the message being sent (**String**) {Text, Binary, Ping, Pong, Close}
    - **send_data** - body of the message (**String**) - optional
    - **close_code** - close code for Close send_type (**String**) - optional, only meaningful if send_type is Close  
    - **response_type** - type of the response message (**String**) - optional {same as send_type}
    - **response_data** - body of the response (**String**) - optional
