use graphql_client::{GraphQLQuery, Response};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

use crate::graphql::{anilist, anime_data, AnimeData, Media};
use crate::{commands::anime::Anime, graphql};

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
            let request = AnimeData::build_query(anime_data::Variables {
                search: Some(anime_name.to_string()),
            });
            let client = reqwest::Client::new();

            let response = client
                .post("https://graphql.anilist.co")
                .json(&request)
                .send()
                .await?;

            let response_body: Response<anime_data::ResponseData> = response.json().await?;

            let data = response_body.data.ok_or("No data found")?;

            let data = serde_json::to_value(data.media).unwrap();
            let media: Media = serde_json::from_value(data).unwrap();

            let anime_json = reqwest::get(&format!(
                "https://kitsu.io/api/edge/anime?filter[text]={}&page[limit]=2",
                anime_name
            ))
            .await?
            .json::<serde_json::Value>()
            .await?;

            let test = serde_json::to_value(&anime_json).unwrap();
            let finals: crate::components::kitsu::Anime = serde_json::from_value(test).unwrap();

            println!("{:#?}", finals);

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
                            .title(format!("Info for {}", name))
                            .description(media.description)
                            .image(media.coverImage.large)
                            .fields(vec![
                                ("Rating", format!("{}", media.averageScore), true),
                                (
                                    "Episode Count",
                                    format!("{} episodes", media.episodes),
                                    true,
                                ),
                                (
                                    "Episode Length",
                                    format!("{} minutes", anime.episode_length),
                                    true,
                                ),
                            ])
                            .fields(vec![
                                (
                                    "Start Date",
                                    format!(
                                        "{} {} {}",
                                        media.startDate.year,
                                        media.startDate.month,
                                        media.startDate.day
                                    ),
                                    true,
                                ),
                                (
                                    "End Date",
                                    format!(
                                        "{} {} {}",
                                        media.endDate.year, media.endDate.month, media.endDate.day
                                    ),
                                    true,
                                ),
                                ("Status", media.status, true),
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
