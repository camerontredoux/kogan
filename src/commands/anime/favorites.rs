use crate::{
    components::anilist::{trending_query, Page},
    services::{init_services, user_service::UpsertUserReq},
};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

#[command]
#[only_in("guild")]
#[description("List all of your favorited animes.")]
async fn favorites(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Check if the command is sent from the correct channel
    // Should only work in the anime channel
    match msg.channel_id.name(&ctx).await {
        Some(m) => {
            if m.as_str() != "anime" {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.description("This command must be used in the `anime` channel.")
                                .color(Color::RED)
                        })
                    })
                    .await?;
                return Ok(());
            }
        }
        None => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description("This command must be used in a valid channel.")
                            .color(Color::RED)
                    })
                })
                .await?;
            return Ok(());
        }
    }

    let user_service = init_services().await.user_service;

    let user = user_service.get_user(msg.author.id.0.to_string()).await?;

    match user {
        Some(user) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("List of favorited animes").field(
                            "Titles",
                            user.animes.join("\n"),
                            false,
                        )
                    })
                })
                .await?;
        }
        None => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Missing data")
                            .description("Cannot find any animes in your favorited list.")
                    })
                })
                .await?;
        }
    }

    Ok(())
}
