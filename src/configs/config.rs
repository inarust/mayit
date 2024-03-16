use mongodb::{Client, options::ClientOptions};
use std::{sync::Arc, env};
use futures::executor;

pub struct AppConfig {
    pub server_address: String,
    pub arc_client: Arc<Client>,
    // ... other configuration parameters ...
}

impl AppConfig {
    pub fn load() -> Result<Self, mongodb::error::Error> {
        // Set up the MongoDB connection options
        let mongouri = env::var("MONGOSTRINGAWANGGA").unwrap_or_default();
        
        // Parse the MongoDB connection options asynchronously
        let client_options_future = ClientOptions::parse(mongouri);
        
        // Block the current thread until the future completes
        let client_options = match executor::block_on(client_options_future) {
            Ok(options) => options,
            Err(err) => return Err(err),
        };
        
        // Connect to the MongoDB server
        let client = Client::with_options(client_options)?;
        let arc_client = Arc::new(client);
        
        // Logic to load configuration from a file, environment variables, etc.
        Ok(AppConfig {
            server_address: env::var("SERVERADDRESS").unwrap_or_default(),
            arc_client,
            // ... initialize other configuration parameters ...
        })
    }
}
