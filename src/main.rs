mod configs;

extern crate may_minihttp;

use std::io;
use may_minihttp::{HttpServer, HttpService, Request, Response};

#[derive(Clone)]
struct HelloWorld;

impl HttpService for HelloWorld {
    fn call(&mut self, _req: Request, res: &mut Response) -> io::Result<()> {
        res.body("Hello, world!");
        Ok(())
    }
}

// Start the server in `main`.
fn main() {
    // Load the configuration
    let config_result = configs::AppConfig::load();

    // Handle the result
    match config_result {
        Ok(config) => {
            // Start the HTTP server with the retrieved server address
            let server = HttpServer(HelloWorld).start(&config.server_address).unwrap();
            server.join().unwrap();
        }
        Err(err) => {
            eprintln!("Failed to load configuration: {:?}", err);
            // Handle the error accordingly
        }
    }
}
