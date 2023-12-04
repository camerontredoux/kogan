mod components;
mod services;

use kogan::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    init(std::env::var("TOKEN").expect("Missing TOKEN in .env"))?;

    Ok(())
}
