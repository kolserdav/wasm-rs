use wasm_bindgen::prelude::*;
extern crate web_sys;
use web_sys::{Document, Window};

#[wasm_bindgen]
pub fn say(s: String) -> String {
    let r = String::from("hello ");
    return r + &s;
}

#[wasm_bindgen]
pub fn say_hello() {
    let window = web_sys::window().expect("Error get window");
    let document = window.document().expect("Error get document");
    let body = document
        .query_selector("body")
        .expect("body not found")
        .unwrap();
    let hello: Option<&str> = Some("Hello Webassembly!");
    body.set_text_content(hello);
}
