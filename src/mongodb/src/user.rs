use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up client options
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    // Get a handle to the "users" collection
    let db = client.database("mydatabase");
    let users = db.collection("users");

    // Insert a document with required fields
    let doc = doc! {
        "_id": ObjectId::new(),
        "name": "Alice",
        "email": "alice@example.com",
        "age": 25
    };

    users.insert_one(doc, None).await?;

    Ok(())
}
