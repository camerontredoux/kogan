use std::time::Duration;

use rand::prelude::SliceRandom;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        gateway::{Activity, Ready},
        id::GuildId,
    },
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Kōgan started and is ready to go 🔥");
        let guild_id = GuildId(
            std::env::var("GUILD_ID")
                .expect("No GUILD_ID found in environment.")
                .parse()
                .expect("Invalid GUILD_ID"),
        );
        tokio::time::sleep(Duration::from_secs(2)).await;
        if let Some(guild) = ctx.cache.guild(guild_id) {
            let members = guild
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
}
