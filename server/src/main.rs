use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

#[cfg(not(target_os = "windows"))]
fn get_file_path(dir: PathBuf, uri: String) -> String {
    format!("{}/dist{:?}", dir.display(), uri.replace("\"", ""))
}

#[cfg(target_os = "windows")]
fn get_file_path(dir: PathBuf, uri: String) -> String {
    format!("{}\\dist{:?}", dir.display(), uri.replace("/", "\\")).replace("\"", "")
}

fn read_file_data<'a>(path: &'a Path) -> Vec<u8> {
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

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let uri = _req.uri().to_string();
    let dir = current_dir().unwrap();

    let path = get_file_path(dir, uri);
    let path = Path::new(&path);
    let chunk_list = read_file_data(&path);

    let ext = path.extension().unwrap();
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
    let body = response.body(Body::from(chunk_list)).unwrap();
    Ok(body)
}

#[tokio::main]
async fn main() {
    for (k, v) in std::env::vars() {
        println!("{}:{}", k, v);
    }

    let addr = SocketAddr::from(([192, 168, 0, 3], 3000));

    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    println!("Listen server at {}", addr);
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
