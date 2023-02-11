use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

pub type Error = std::io::Error;
pub type ErrorKind = std::io::ErrorKind;

fn doc() -> Option<Document> {
    let window = web_sys::window()?;
    window.document()
}

pub fn el(selector: &str) -> Result<Element, JsValue> {
    let doc_o = doc();
    if let None = &doc_o {
        return Err(JsValue::from(format!("document is missing: {}", selector)));
    };
    let _doc = doc_o.unwrap();
    let el_o = _doc.query_selector(selector)?;
    if let Some(v) = el_o {
        return Ok(v);
    };
    Err(JsValue::from(format!("element is missing: {}", selector)))
}
