use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

#[command]
async fn track(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
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

            let name = anime_json["data"][0]["attributes"]["canonicalTitle"]
                .as_str()
                .unwrap();
            let description = anime_json["data"][0]["attributes"]["synopsis"]
                .as_str()
                .unwrap();
            let image_url = anime_json["data"][0]["attributes"]["posterImage"]["small"]
                .as_str()
                .unwrap();
            // let rating = anime_json["data"][0]["attributes"]["averageRating"]
            //     .as_f64()
            //     .unwrap();
            // let episodes = anime_json["data"][0]["attributes"]["episodeCount"]
            //     .as_i64()
            //     .unwrap();
            // let start_date = anime_json["data"][0]["attributes"]["startDate"]
            //     .as_str()
            //     .unwrap();
            // let end_date = anime_json["data"][0]["attributes"]["endDate"]
            //     .as_str()
            //     .unwrap();
            // let status = anime_json["data"][0]["attributes"]["status"]
            //     .as_str()
            //     .unwrap();
            // let genres = anime_json["data"][0]["attributes"]["genres"]
            //     .as_array()
            //     .unwrap();
            // let studios = anime_json["data"][0]["attributes"]["studios"]
            //     .as_array()
            //     .unwrap();
            // let episode_length = anime_json["data"][0]["attributes"]["episodeLength"]
            //     .as_i64()
            //     .unwrap();

            println!("name: {}", name);

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(Color::DARK_GREEN)
                            .title(format!("Tracking {}", name))
                            .description(description)
                            .image(image_url)
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

    Ok(())
}
