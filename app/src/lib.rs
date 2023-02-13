use wasm_bindgen::prelude::*;
extern crate web_sys;
use web_sys::console;
mod prelude;
use gloo_utils::format::JsValueSerdeExt;
use js_sys::Function;
use prelude::{el, Error, ErrorKind};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use web_sys::EventListener;

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
    content_editable();

    // hello_webassembly();
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

fn content_editable() {
    let t = el("#html");
    if let Err(e) = t {
        console::log_1(&e);
        return;
    }
    let t = t.unwrap();
    console::log_1(&JsValue::from(&t));
    let listener = EventListener::new();
    let cb = Function::new_no_args("");
    let list = t.add_event_listener_with_event_listener("select", &listener);
}

fn hello_webassembly() -> Result<(), JsError> {
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
    Ok(())
}
