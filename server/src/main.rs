use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::env::current_dir;
use std::net::SocketAddr;
mod prelude;
use prelude::*;
use std::path::Path;

const INDEX_PAGE: &str = "/index.html";
const HOST: [u8; 4] = [192, 168, 0, 3];
const PORT: u16 = 3000;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut uri = _req.uri().to_string();
    uri = if uri == "/" {
        INDEX_PAGE.to_string()
    } else {
        uri
    };

    let dir = current_dir().unwrap();

    let path = get_file_path(dir, uri);
    let path = Path::new(&path);
    let chunk_list = read_file_data(&path);

    let ext = path.extension().unwrap();
    let res = create_response(&ext, chunk_list);
    if let Err(e) = res {
        println!("error response: {:?}: by request: {:?}", e, _req);
        let body = Body::from("Error".to_string());
        let res = Response::new(body);
        return Ok(res);
    }
    let res = res.unwrap();
    Ok(res)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from((HOST, PORT));

    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    println!("Listen server at {}", addr);
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
