use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

use super::trending::*;

struct Anime<'a> {
    name: &'a str,
    image_url: &'a str,
    description: &'a str,
    rating: &'a str,
    episodes: i64,
    episode_length: i64,
    start_date: &'a str,
    end_date: &'a str,
    status: &'a str,
    age_rating: &'a str,
    age_rating_guide: &'a str,
}

impl<'a> Anime<'a> {
    fn new(anime_json: &'a serde_json::Value) -> Self {
        let name = anime_json["data"][0]["attributes"]["canonicalTitle"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let description = anime_json["data"][0]["attributes"]["synopsis"]
            .as_str()
            .unwrap_or_else(|| "No description available");
        let image_url = anime_json["data"][0]["attributes"]["posterImage"]["small"]
            .as_str()
            .unwrap_or_else(|| "No image available");
        let rating = anime_json["data"][0]["attributes"]["averageRating"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let episodes = anime_json["data"][0]["attributes"]["episodeCount"]
            .as_i64()
            .unwrap_or_else(|| 0);
        let start_date = anime_json["data"][0]["attributes"]["startDate"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let end_date = anime_json["data"][0]["attributes"]["endDate"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let status = anime_json["data"][0]["attributes"]["status"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let episode_length = anime_json["data"][0]["attributes"]["episodeLength"]
            .as_i64()
            .unwrap_or_else(|| 0);
        let age_rating = anime_json["data"][0]["attributes"]["ageRating"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let age_rating_guide = anime_json["data"][0]["attributes"]["ageRatingGuide"]
            .as_str()
            .unwrap_or_else(|| "N/A");

        let status = match status {
            "finished" => "Finished",
            "current" => "Current",
            "tba" => "To Be Announced",
            "unreleased" => "Unreleased",
            "upcoming" => "Upcoming",
            _ => "Unknown",
        };

        Anime {
            name,
            image_url,
            description,
            rating,
            episodes,
            episode_length,
            start_date,
            end_date,
            status,
            age_rating,
            age_rating_guide,
        }
    }
}

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

            let anime = Anime::new(&anime_json);

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
