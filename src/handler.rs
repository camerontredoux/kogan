use std::{str::FromStr, time::Duration};

use rand::prelude::SliceRandom;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    futures::StreamExt,
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        id::GuildId,
        interactions::InteractionResponseType,
    },
    utils::Color,
};

use crate::components::{animal::Animal, sounds::Sound};

pub struct Handler;

async fn update_status(ctx: &Context, guild_id: GuildId) {
    println!("guild_id: {}", guild_id);
    tokio::time::sleep(Duration::from_secs(2)).await;
    let members = ctx.cache.guild(guild_id);
    if let Some(members) = members {
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
    } else {
        ctx.set_activity(Activity::playing("with no one 🥲")).await;
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Kōgan started and is ready to go 🔥");

        let log = std::env::args().skip(1).collect::<Vec<_>>();
        let guild_id = GuildId(std::env::var("GUILD_ID").unwrap().parse().unwrap());

        match log.get(0) {
            Some(log) if log == "--log" => {
                let channels = ctx.cache.guild_channels(guild_id).unwrap();

                let chrono = chrono::Local::now();
                let time = chrono.format("%Y-%m-%d %I:%M:%S %p").to_string();
                let bot_logs = channels.iter().find(|c| c.name() == "bot-logs").unwrap();
                if let Err(err) = bot_logs
                    .send_message(&ctx, |m| {
                        m.embed(|e| {
                            e.color(Color::DARK_GREEN)
                                .title(format!("Kōgan started! {}", time))
                        })
                    })
                    .await
                {
                    println!("Error sending message: {:?}", err);
                }
            }
            _ => {}
        }
        update_status(&ctx, guild_id).await;
    }

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
