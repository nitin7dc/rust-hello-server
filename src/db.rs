use mongodb::{Client, options::ClientOptions, Database};
use std::error::Error;

pub async fn init_db() -> Result<Database, Box<dyn Error>> {

    // Parse the MongoDB connection string
    let client_options = ClientOptions::parse(std::env::var("MONGODB_URI").unwrap()).await?;

    // Create the MongoDB client
    let client = Client::with_options(client_options)?;

    let database_name = std::env::var("DB_NAME").unwrap();
    // Get the database
    Ok(client.database(&database_name))

}