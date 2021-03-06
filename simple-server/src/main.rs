extern crate env_logger;
#[macro_use]
extern crate log;

extern crate simple_server;

use simple_server::Server;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let host = "127.0.0.1";
    let port = "7878";

    let server = Server::new(|request, mut response| {
        info!("Request received. {} {}", request.method(), request.uri());
        Ok(response.body("{\"value\":\"pong\"}".as_bytes().to_vec())?)
    });

    server.listen(host, port);
}
