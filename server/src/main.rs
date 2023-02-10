use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::path::Path;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let uri = _req.uri().to_string();
    let dir = current_dir().unwrap();
    let path = format!("{}\\dist{:?}", dir.display(), uri.replace("/", "\\")).replace("\"", "");
    let path = Path::new(&path);
    println!("path: {:?} file: {:?}", path, uri);
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
    Ok(Response::new(Body::from(chunk_list)))
}

#[tokio::main]
async fn main() {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([192, 168, 0, 3], 3000));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    println!("Listen server at {}", addr);
    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
