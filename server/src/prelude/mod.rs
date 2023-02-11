use hyper::{http, Body, Response};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[cfg(not(target_os = "windows"))]
pub fn get_file_path(dir: PathBuf, uri: String) -> String {
    format!("{}/dist{:?}", dir.display(), uri.replace("\"", ""))
}

#[cfg(target_os = "windows")]
pub fn get_file_path(dir: PathBuf, uri: String) -> String {
    format!("{}\\dist{:?}", dir.display(), uri.replace("/", "\\")).replace("\"", "")
}

pub fn read_file_data<'a>(path: &'a Path) -> Vec<u8> {
    println!("path: {:?}", path);
    let mut f = File::open(path).unwrap();
    let mut chunk_list = Vec::<u8>::new();
    loop {
        let mut chunk = [0; 1];
        let l = f.read(&mut chunk).unwrap();
        if l == 0 {
            break;
        }
        chunk.map(|d| {
            chunk_list.push(d);
        });
    }
    chunk_list
}

pub fn create_response(ext: &OsStr, chunk_list: Vec<u8>) -> Result<Response<Body>, http::Error> {
    let mut response = Response::builder();
    let mut content_type = "text/html";
    if ext == "wasm" {
        content_type = "application/wasm";
    } else if ext == "js" {
        content_type = "application/javascript";
    } else if ext == "css" {
        content_type = "text/css";
    }
    response = response.header("Content-Type", content_type);
    response.body(Body::from(chunk_list))
}
