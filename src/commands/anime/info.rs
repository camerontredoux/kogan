use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

#[command]
#[only_in("guilds")]
#[description("Gathers data for an anime using kitsu.io and displays relevant information.")]
#[usage("<anime name>")]
#[example("aot season 4")]
#[example("attack on titan")]
#[example("shingeki final season")]
async fn info(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
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

    let anime_name = args.remains();

    match anime_name {
        Some(anime_name) => {
            // let client = reqwest::Client::new();

            // let response = client
            //     .post("https://graphql.anilist.co")
            //     .json(&request)
            //     .send()
            //     .await?;

            // let response_body: Response<anime_data::ResponseData> = response.json().await?;

            // let data = response_body.data.ok_or("No data found")?;

            // let data = serde_json::to_value(data.media).unwrap();
            // let media: Media = serde_json::from_value(data).unwrap();

            let anime_json = reqwest::get(&format!(
                "https://kitsu.io/api/edge/anime?filter[text]={}&page[limit]=1",
                anime_name
            ))
            .await?
            .json::<serde_json::Value>()
            .await?;

            let test = serde_json::to_value(&anime_json).unwrap();
            let finals: crate::components::kitsu::Anime = serde_json::from_value(test).unwrap();

            let anime = match finals.data.get(0) {
                Some(anime) => &anime.attributes,
                None => {
                    msg.channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.description("No anime found with that title.")
                                    .color(Color::RED)
                            })
                        })
                        .await?;
                    return Ok(());
                }
            };

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(Color::DARK_GREEN)
                            .title(format!("Info for {}", anime.canonical_title()))
                            .description(anime.synopsis())
                            .attachment(anime.poster_image())
                            .image(anime.poster_image())
                            .fields(vec![
                                ("Rating", format!("{}", anime.average_rating()), true),
                                (
                                    "Episode Count",
                                    format!("{} episodes", anime.episode_count()),
                                    true,
                                ),
                                (
                                    "Episode Length",
                                    format!("{} minutes", anime.episode_length()),
                                    true,
                                ),
                            ])
                            .fields(vec![
                                ("Start Date", anime.start_date(), true),
                                ("End Date", anime.end_date(), true),
                                ("Status", anime.status(), true),
                            ])
                            .field("Age Rating Guide", anime.age_rating_guide(), false)
                            .footer(|f| f.text("Powered by Kitsu.io"))
                    })
                })
                .await?;
        }
        None => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                            e.description("No anime name provided. Use command with `<anime name>` or check the help command for more information.")
                        .color(Color::RED)
                    })
                })
                .await?;
        }
    }

    Ok(())
}
