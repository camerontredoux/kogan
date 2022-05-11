use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

use crate::commands::anime::Anime;

use super::trending::*;

#[command("info")]
#[only_in("guilds")]
#[description("Gathers data for an anime using kitsu.io and displays relevant information.")]
#[usage("<anime name>")]
#[example("aot season 4")]
#[example("attack on titan")]
#[example("shingeki final season")]
#[sub_commands(trending)]
async fn info(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    match msg.channel_id.name(&ctx).await {
        Some(m) => {
            if m.as_str() != "anime" {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.description("This command must be used in the `anime` channel")
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
                        e.description("This command must be used in a server.")
                            .color(Color::RED)
                    })
                })
                .await?;
            return Ok(());
        }
    }

    let anime_name = args.remains();

    match anime_name {
        Some(anime_name) => {
            let anime_json = reqwest::get(&format!(
                "https://kitsu.io/api/edge/anime?filter[text]={}&page[limit]=1",
                anime_name
            ))
            .await?
            .json::<serde_json::Value>()
            .await?;

            let anime = Anime::new(&anime_json, 0);

            let name = match anime.name {
                "N/A" => {
                    msg.channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| e.description("Anime not found").color(Color::RED))
                        })
                        .await?;
                    return Ok(());
                }
                _ => anime.name,
            };

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(Color::DARK_GREEN)
                            .title(format!("Info for {} (Rated {})", name, anime.age_rating))
                            .description(anime.description)
                            .image(anime.image_url)
                            .fields(vec![
                                ("Rating", anime.rating, true),
                                (
                                    "Episode Count",
                                    format!("{} episodes", anime.episodes).as_str(),
                                    true,
                                ),
                                (
                                    "Episode Length",
                                    format!("{} minutes", anime.episode_length).as_str(),
                                    true,
                                ),
                            ])
                            .fields(vec![
                                ("Start Date", anime.start_date, true),
                                ("End Date", anime.end_date, true),
                                ("Status", anime.status, true),
                            ])
                            .field("Age Rating Guide", anime.age_rating_guide, false)
                            .footer(|f| f.text("Powered by Kitsu.io"))
                    })
                })
                .await?;
        }
        None => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| e.color(Color::RED).description("No anime name provided."))
                })
                .await?;
        }
    }

    Ok(())
}
