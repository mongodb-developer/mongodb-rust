use mongodb::{options::ClientOptions, Collection, bson::Document};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   // Create a client that connects to the computer_scientists collection
   let options = ClientOptions::parse("mongodb+srv://blog:blog@cluster0.2grje.mongodb.net/test?authSource=admin&replicaSet=atlas-v6xmes-shard-0&readPreference=primary&appname=MongoDB%20Compass&ssl=true").await?;
   let client = mongodb::Client::with_options(options)?;
   let theaters: Collection<Document> = client.database("sample_mflix").collection("theaters");
   // Look up one document:
   let theater = theaters.find_one(None, None,).await?.expect("No matching documents found.");
   println!("Theater: {}", theater);

   Ok(())
}
