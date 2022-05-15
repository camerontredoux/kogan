use handler::Handler;

use std::process;

mod graphql;

mod handler;

mod commands;
use commands::anime::announce::*;
use commands::anime::info::*;
use commands::anime::trending::*;
use commands::help::*;
use commands::rules::*;

mod hooks;
use hooks::*;

mod components;

use serenity::{
    framework::standard::macros::group,
    framework::{standard::buckets::LimitedFor, StandardFramework},
    http::Http,
    prelude::*,
    Client,
};

#[group]
#[description = "General commands"]
#[commands(rules, announce, info, trending)]
struct General;

#[tokio::main]
pub async fn init(token: String) {
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
        .configure(|c| c.prefix(".").with_whitespace(true).on_mention(Some(bot_id)))
        .help(&HELP)
        .unrecognised_command(unknown_command)
        .bucket("emoji", |b| {
            b.delay(5)
                .delay_action(delay_action)
                .limit_for(LimitedFor::User)
        })
        .await
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
