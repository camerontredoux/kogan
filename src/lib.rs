mod commands;
mod components;
mod handler;
mod hooks;

use commands::{
    about::*,
    anime::{countdown::*, info::*, trending::*},
    help::*,
};
use handler::Handler;
use hooks::*;

use serenity::{
    framework::standard::macros::group,
    framework::{standard::buckets::LimitedFor, StandardFramework},
    http::Http,
    prelude::*,
    Client,
};
use std::process;

#[group]
#[description = "Kōgan's general commands for displaying anime information."]
#[commands(info, trending, countdown)]
struct Anime;

#[group]
#[description = "Kōgan's commands for displaying bot information."]
#[commands(about)]
struct About;

#[tokio::main]
pub async fn init(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_PRESENCES
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
        .configure(|c| {
            c.prefix(".")
                .case_insensitivity(true)
                .with_whitespace(true)
                .on_mention(Some(bot_id))
        })
        .help(&HELP)
        .unrecognised_command(unknown_command)
        .bucket("emoji", |b| {
            b.delay(5)
                .delay_action(delay_action)
                .limit_for(LimitedFor::User)
        })
        .await
        .group(&ANIME_GROUP)
        .group(&ABOUT_GROUP);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await?;

    if let Err(err) = client.start().await {
        println!("Error starting client: {}", err);
    }

    Ok(())
}
