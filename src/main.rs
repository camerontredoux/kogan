use kogan::*;

fn main() {
    dotenv::dotenv().ok();
    init(std::env::var("TOKEN").expect("Missing TOKEN in .env"));
}
