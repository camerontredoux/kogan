mod components;
mod services;

use kogan::*;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use services::user_service::{self, User};
use services::ServiceContainer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    init(std::env::var("TOKEN").expect("Missing TOKEN in .env"))?;

    let client_uri = std::env::var("MONGODB_URI").expect("Missing MONGODB_URI in .env");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;

    let client = Client::with_options(options)?;

    Ok(())
}

fn init_services(client: &Client) -> ServiceContainer {
    let user_collection = client.database("kogan").collection::<User>("users");

    ServiceContainer {
        user_service: user_service::UserService::new(user_collection),
    }
}
