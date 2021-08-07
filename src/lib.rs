extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use web_sys::{Request, Response, ResponseInit };
use std::collections::HashMap;


use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use url::Url;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

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

pub fn string() -> Result<Response, JsValue> {
    let mut init = ResponseInit::new();
    let mut headers: HashMap<&str, &str> = HashMap::new();
    headers.insert("Content-Type", "text/plain");
    headers.insert("Cache-Control", "no-cache");

    let headers = wasm_bindgen::JsValue::from_serde(&headers).unwrap();
    init.headers(&headers);
    init.status(200);

    web_sys::Response::new_with_opt_str_and_init(Some("Hello world"), &init)
}

pub fn not_found() -> Result<Response, JsValue> {
    let mut init = web_sys::ResponseInit::new();
    let mut headers: HashMap<&str, &str> = HashMap::new();
    headers.insert("Content-Type", "text/plain");
    headers.insert("Cache-Control", "no-cache");

    let headers = wasm_bindgen::JsValue::from_serde(&headers).unwrap();
    init.headers(&headers);
    init.status(404);

    web_sys::Response::new_with_opt_str_and_init(Some("404 - page not found"), &init)
}

#[wasm_bindgen]
pub fn execute(req: Request) -> Response {
    let url = Url::parse(&req.url()).unwrap();
    let path_parts: Vec<&str> = url.path_segments()
        .map_or(Vec::new(), |f| f.collect());

    match path_parts.as_slice() {
        ["bytes_a"] => response_as_bytes(String::from("Hello world!")),
        ["bytes_b"] => response_as_bytes(String::from("Hello world!!!!!!!!!!")),
        ["bytes_c"] => response_as_bytes(String::from("Hello worldddddddddd")),
        ["bytes_d"] => response_as_bytes_debug(String::from("Hello world!")),
        ["string"] => string(),
        _ => not_found()
    }.unwrap()
}
