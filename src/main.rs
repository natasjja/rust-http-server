use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let ip: String = "127.0.0.1:8080".to_string();

    let server = Server::new(ip);
    server.run();
}
