use kogan::*;

mod graphql;

#[tokio::main]
async fn main() {
    graphql::anilist::make_request();
    dotenv::dotenv().ok();
    init(std::env::var("TOKEN").expect("Missing TOKEN in .env"));
}
