use std::process;
use std::str::FromStr;
use std::time::Duration;

mod commands;
use commands::emoji::*;
use commands::help::*;
use commands::rules::*;
use commands::track::announce::*;
use commands::track::track::*;

mod hooks;
use components::sounds::Sound;
use hooks::*;

mod components;
use components::animal::*;

use serenity::futures::StreamExt;
use serenity::model::interactions::InteractionResponseType;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::standard::macros::group,
    framework::{standard::buckets::LimitedFor, StandardFramework},
    http::Http,
    model::channel::Message,
    model::{gateway::Ready, interactions::Interaction},
    prelude::*,
    Client,
};

#[group]
#[description = "General commands"]
#[commands(rules, announce, track)]
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
        println!("Kōgan started and is ready to go 🔥");
    }

    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {}

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "animal" {
            let m = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.content("Please select your favorite animal")
                        .components(|c| c.add_action_row(Animal::action_row()))
                })
                .await
                .unwrap();

            let mci = match m
                .await_component_interaction(&ctx)
                .timeout(Duration::from_secs(3 * 60))
                .await
            {
                Some(c) => c,
                None => {
                    m.reply(&ctx.http, "Timed out").await.unwrap();
                    return;
                }
            };

            let animal = Animal::from_str(mci.data.values.get(0).unwrap()).unwrap();

            mci.create_interaction_response(&ctx.http, |r| {
                r.kind(InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|d| {
                        d.content(format!("You chose **{}**, now choose a sound.", animal))
                            .components(|c| c.add_action_row(Sound::action_row()))
                    })
            })
            .await
            .unwrap();

            let mut cib = m
                .await_component_interactions(&ctx)
                .timeout(Duration::from_secs(60 * 3))
                .build();

            while let Some(mci) = cib.next().await {
                let sound = Sound::from_str(&mci.data.custom_id).unwrap();
                // Acknowledge the interaction and send a reply
                mci.create_interaction_response(&ctx, |r| {
                    // This time we dont edit the message but reply to it
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| {
                            // Make the message hidden for other users by setting `ephemeral(true)`.
                            d.ephemeral(true)
                                .content(format!("The **{}** says __{}__", animal, sound))
                        })
                })
                .await
                .unwrap();
            }

            m.delete(&ctx).await.unwrap();
        }
    }
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
