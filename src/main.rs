#[allow(unused_imports)]
use http::Method;
#[allow(unused_imports)]
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:80".to_string());
    server.run();
}
