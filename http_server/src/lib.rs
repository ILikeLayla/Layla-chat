mod router;
mod server;
mod handler;

use server::Server;
use dotenv::dotenv;

fn run() {
    dotenv().ok();
    let server = Server::new("127.0.0.1:3000");
    server.run();
}
