pub mod game;
pub mod menu;
pub mod user;
use mongodb::{options::ClientOptions, Client};
use mongodb::{Collection, Database};
use std::error::Error;
//获取数据库连接
async fn get_connect_database(database_name: &str) -> Result<Database, Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    // Parse a connection string into an options struct.
    println!("load mongodb.......");
    let mut client_options =
        ClientOptions::parse("mongodb://root:123456@localhost:27017/cake?authSource=admin").await?;
    // Manually set an option.
    client_options.app_name = Some("game-menu".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    println!("Get a game-menu database ...");
    // Get a handle to a database.
    let db = client.database(database_name);

    Ok(db)
}
//获取collection的handle
pub async fn get_handle_with_collection<T>(
    database_name: &str,
    collection_name: &str,
) -> Result<Collection<T>, Box<dyn Error>> {
    let db = match get_connect_database(database_name).await {
        Ok(v) => {
            println!("Connected to database {} ......", database_name);
            v
        }
        Err(e) => panic!(
            "Connect to database {:?} has some errors: {:?}",
            database_name, e
        ),
    };
    // Get a handle to a collection in the database.
    let collection = db.collection::<T>(collection_name);
    Ok(collection)
}
