use rand;
use rand::prelude::SliceRandom;

use std::ops::DerefMut;
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
use serenity::model::gateway::Activity;
use serenity::model::id::GuildId;
use serenity::model::interactions::InteractionResponseType;
use serenity::utils::Color;
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

async fn update_status(ctx: &Context, guild_id: GuildId) {
    let members = ctx.cache.guild(guild_id).unwrap();
    let members = members
        .members
        .values()
        .filter(|m| !m.user.bot)
        .collect::<Vec<_>>();

    loop {
        let random_member = members
            .choose(&mut rand::thread_rng())
            .unwrap()
            .display_name()
            .to_string();

        ctx.set_activity(Activity::playing(format!("with {} 🔥", random_member)))
            .await;
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Kōgan started and is ready to go 🔥");

        let guild_id = GuildId(std::env::var("GUILD_ID").unwrap().parse().unwrap());

        let channels = ctx.cache.guild_channels(guild_id).unwrap();

        let bot_logs = channels.iter().find(|c| c.name() == "bot-logs").unwrap();
        if let Err(err) = bot_logs
            .send_message(&ctx, |m| {
                m.embed(|e| e.color(Color::DARK_GREEN).title("Kōgan started!"))
            })
            .await
        {
            println!("Error sending message: {:?}", err);
        }

        update_status(&ctx, guild_id).await;
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
