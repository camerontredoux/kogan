use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{gateway::Ready, interactions::Interaction},
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data: Ready) {}

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {}
}

#[tokio::main]
pub async fn init(token: String) {
    println!("{}", token);
}
