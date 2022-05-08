use std::process;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            CommandResult,
        },
        Framework, StandardFramework,
    },
    http::Http,
    model::{channel::Message, gateway::Ready, interactions::Interaction},
    prelude::GatewayIntents,
    Client,
};

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data: Ready) {}

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {}
}

#[tokio::main]
pub async fn init(token: String) {
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;
    let http = Http::new(&token);
    let bot_id = match http.get_current_application_info().await {
        Ok(_) => match http.get_current_user().await {
            Ok(bot_id) => bot_id.id,
            Err(err) => {
                println!("Failed to access bot id: {}", err);
                process::exit(1);
            }
        },
        Err(err) => {
            println!("Failed to access bot info: {}", err);
            process::exit(1);
        }
    };
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(".").with_whitespace(true).on_mention(Some(bot_id)))
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(err) = client.start().await {
        println!("Failed to start client: {}", err);
        process::exit(1);
    };
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "content").await?;

    Ok(())
}
