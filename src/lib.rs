use std::process;

mod commands;
use commands::emoji::*;
use commands::help::*;
use commands::rules::*;

mod hooks;
use hooks::*;

use serenity::framework::standard::macros::group;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::{standard::buckets::LimitedFor, StandardFramework},
    http::Http,
    model::{gateway::Ready, interactions::Interaction},
    prelude::GatewayIntents,
    Client,
};

#[group]
#[description = "General commands"]
#[commands(rules)]
struct General;

#[group]
#[prefix = "emoji"]
#[description = "Emoji messages"]
#[commands(cat)]
struct Emoji;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _: Ready) {
        println!("Kōgan started and ready.");
    }

    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {}
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
        .help(&HELP)
        .unrecognised_command(unknown_command)
        .bucket("emoji", |b| {
            b.delay(5)
                .delay_action(delay_action)
                .limit_for(LimitedFor::User)
        })
        .await
        .group(&EMOJI_GROUP)
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
