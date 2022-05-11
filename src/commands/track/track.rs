use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

#[command]
async fn track(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
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

            let name = anime_json["data"][0]["attributes"]["canonicalTitle"].as_str();

            let name = match name {
                Some(name) => name,
                None => {
                    msg.channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| e.description("Anime not found").color(Color::RED))
                        })
                        .await?;
                    return Ok(());
                }
            };

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
                .unwrap_or_else(|| 0)
                .to_string();
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

            let status = match status {
                "finished" => "Finished",
                "current" => "Current",
                "tba" => "To Be Announced",
                "unreleased" => "Unreleased",
                "upcoming" => "Upcoming",
                _ => "Unknown",
            };

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(Color::DARK_GREEN)
                            .title(format!("Tracking {}", name))
                            .description(description)
                            .image(image_url)
                            .fields(vec![
                                ("Rating", rating, true),
                                ("Episode Count", episodes.as_str(), true),
                                (
                                    "Episode Length",
                                    format!("{} min.", episode_length).as_str(),
                                    true,
                                ),
                            ])
                            .fields(vec![
                                ("Start Date", start_date, true),
                                ("End Date", end_date, true),
                                ("Status", status, true),
                            ])
                            .footer(|f| f.text("Powered by Kitsu.io"))
                    })
                })
                .await?;
        }
        None => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(Color::RED)
                            .title("Track failed!")
                            .description("No anime name provided.")
                    })
                })
                .await?;
        }
    }

    Ok(())
}
