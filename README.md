# POC Cloudflare Workers WASM web_sys u8 response issues

This is a poc test case using web_sys `Response::new_with_opt_u8_array_and_init` with Cloudflare workers.

I had an application on Cloudflare workers creating ProtoBuff responses. 
My application was consistently intermittently failing to parse the ProtoBuff responses from my worker which where returned using `Response::new_with_opt_u8_array_and_init`.
On investigating sometimes the binary responses are returned corrupt from my Worker which explains the parse failure.

This is a simplified test case using `new_with_opt_u8_array_and_init` to return a byte encoded string which is easier to debug the protobuf.

The endpoints `/bytes_a`, `/bytes_b`, `bytes_c` from the worker test case in `src/lib.js` consistently return garbled output.

This looks to be something I'm doing wrong on the WASM side with web_sys or a bug with web_sys.

This plain js in a worker works as expected:
```javascript
const encoder = new TextEncoder()
const view = encoder.encode('Hello World')
return new Response(view)
```

Likewise this is the u8 array produced of the encoded string:

```javascript
const encoder = new TextEncoder()
const view = encoder.encode('Hello World!')
console.log(view)

int8Array(12) [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]
```

From `/bytes_a` we can see the body is complete different to above using 
```rust
pub fn response_as_bytes(s: String) -> Result<Response, JsValue> {
    let mut init = ResponseInit::new();
    let mut headers: HashMap<&str, &str> = HashMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8");
    headers.insert("Cache-Control", "no-cache");

    let headers = wasm_bindgen::JsValue::from_serde(&headers).unwrap();
    init.headers(&headers);

    let mut body = s.into_bytes();

    web_sys::Response::new_with_opt_u8_array_and_init(Some(&mut body), &init)
}
```

Results in a byte array of below on the JS side
```
await instance;
const resp = await execute(request)
const body = await resp.arrayBuffer()
console.log(body)
    
[[Uint8Array]]: Uint8Array(12)

    0: 16
    1: 109
    2: 19
    3: 0
    4: 16
    5: 109
    6: 19
    7: 0
    8: 32
    9: 0
    10: 0
    11: 0  
```

To verify if it is an issue with the way I am encoding bytes I produced a JSON array of bytes to debug:
```rust
pub fn response_as_bytes_debug(s: String) -> Result<Response, JsValue> {
    let mut init = ResponseInit::new();
    let mut headers: HashMap<&str, &str> = HashMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8");
    headers.insert("Cache-Control", "no-cache");

    let headers = wasm_bindgen::JsValue::from_serde(&headers).unwrap();
    init.headers(&headers);

    let mut body = s.into_bytes();
    let body = serde_json::to_string(&body).unwrap();

    web_sys::Response::new_with_opt_str_and_init(Some(&body), &init)
}
```

This results in the following below response which is the same as we get for `new TextEncoder().encode("Hello World!)`.
```json
[72,101,108,108,111,32,119,111,114,108,100,33]
```

So it looks like I am correctly generating a utf8 string -> byte array but something isn't right with `web_sys::Response::new_with_opt_u8_array_and_init`.
