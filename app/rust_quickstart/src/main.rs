use mongodb::{options::ClientOptions, Collection, bson::Document, bson::doc};
use std::error::Error;
use tokio;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Create a client that connects to the computer_scientists collection
  let options = ClientOptions::parse("<CONN_STRING>").await?;
  let client = mongodb::Client::with_options(options)?;
  let theaters: Collection<Document> = client.database("sample_mflix").collection("theaters");
  // Look up one document:
  let theater = theaters
                  .find_one(doc! {"theaterId": 1043}, None,)
                  .await
                  ?.expect("No matching documents found.");
  println!("Theater: {}", theater);
 
  Ok(())
}
