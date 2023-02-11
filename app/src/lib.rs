use wasm_bindgen::prelude::*;
extern crate web_sys;
mod prelude;
use gloo_utils::format::JsValueSerdeExt;
use prelude::{el, Error, ErrorKind};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[wasm_bindgen]
pub fn say(s: String) -> String {
    let r = String::from("hello ");
    return r + &s;
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct SayHello {
    first: u8,
    second: bool,
}

#[wasm_bindgen]
pub fn say_hello() -> Result<JsValue, JsError> {
    let body_o = el("body");
    if let Err(e) = body_o {
        return Err(JsError::from(Error::new(
            ErrorKind::NotFound,
            format!("Error: {:?}", e),
        )));
    }
    let body = body_o.unwrap();
    let hello: Option<&str> = Some("Hello Webassembly!");
    body.set_text_content(hello);
    let data = SayHello {
        first: 244,
        second: true,
    };
    let res_r = JsValue::from_serde(&data);
    if let Err(e) = res_r {
        return Err(JsError::from(Error::new(
            ErrorKind::Interrupted,
            format!("Error: {:?}", e),
        )));
    }
    let res = res_r.unwrap();
    Ok(res)
}
