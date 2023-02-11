use wasm_bindgen::prelude::*;
extern crate web_sys;
use web_sys::console;
mod prelude;
use prelude::{el, Error, ErrorKind};

#[wasm_bindgen]
pub fn say(s: String) -> String {
    let r = String::from("hello ");
    return r + &s;
}

#[wasm_bindgen]
pub fn say_hello() -> Result<JsValue, JsError> {
    let body_o = el("b2ody");
    if let Err(e) = body_o {
        return Err(JsError::from(Error::new(ErrorKind::NotFound, "err")));
    }
    let body = body_o.unwrap();
    let hello: Option<&str> = Some("Hello Webassembly!");
    body.set_text_content(hello);
    Ok(JsValue::from("sucess"))
}
