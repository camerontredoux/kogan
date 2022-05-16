use crate::components::anilist::{trending_query, Page, TrendingQuery};
use graphql_client::{GraphQLQuery, Response};
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
            let client = reqwest::Client::new();
            let variables = trending_query::Variables {
                search: Some(anime_name.to_string()),
                amt: Some(1),
                sort: Some(vec![Some(trending_query::MediaSort::POPULARITY_DESC)]),
            };
            let request = TrendingQuery::build_query(variables);

            let response = client
                .post("https://graphql.anilist.co")
                .json(&request)
                .send()
                .await?;

            let response_body: Response<trending_query::ResponseData> = response.json().await?;

            let data = response_body.data.ok_or("No data found")?;

            let data = serde_json::to_value(data.page).unwrap();
            println!("{:#?}", data);
            let page: Page = serde_json::from_value(data).unwrap();
            let media = match page.media.get(0) {
                Some(m) => m,
                None => {
                    msg.channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| e.description("No anime found.").color(Color::RED))
                        })
                        .await?;
                    return Ok(());
                }
            };

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
                        e.color(Color::GOLD)
                            .title(format!(
                                "{} (_{}_)",
                                media.title_romaji(),
                                media.title_english()
                            ))
                            .author(|a| {
                                a.icon_url("https://upload.wikimedia.org/wikipedia/commons/7/7a/MyAnimeList_Logo.png").name("❯❯ Link to MyAnimeList").url(format!("https://myanimelist.net/anime/{}", media.id()))
                            })
                            .description(media.description())
                            .image(media.cover_image())
                            .fields(vec![
                                ("Rating", format!("{}%", media.average_score()), true),
                                (
                                    "Episode Count",
                                    format!("{} episodes", media.episodes()),
                                    true,
                                ),
                                (
                                    "Episode Duration",
                                    format!("{} minutes", media.duration()),
                                    true,
                                ),
                            ])
                            .fields(vec![
                                ("Start Date", media.start_date(), true),
                                ("End Date", media.end_date(), true),
                                ("Status", media.status(), true),
                            ])
                            .fields(vec![
                                (
                                    "Season",
                                    format!("{} {}", media.season(), media.season_year()),
                                    true,
                                ),
                                ("Rankings", media.rankings(), true),
                                ("Popularity", media.popularity(), true),
                            ])
                            .fields(vec![("Genre", media.genres(), true),("Format", media.format(), true)])
                            .footer(|f| {
                                f.text("Powered by AniList.co");
                                f.icon_url(
                                    "https://anilist.co/img/icons/android-chrome-512x512.png",
                                )
                            })
                    })
                })
                .await?;
            // msg.channel_id
            //     .send_message(&ctx.http, |m| {
            //         m.embed(|e| {
            //             e.color(Color::DARK_GREEN)
            //                 .title(format!(
            //                     "Info for {} (Rated {})",
            //                     anime.canonical_title(),
            //                     anime.age_rating()
            //                 ))
            //                 .description(anime.synopsis())
            //                 .image(anime.poster_image())
            //                 .fields(vec![
            //                     ("Rating", format!("{}", anime.average_rating()), true),
            //                     (
            //                         "Episode Count",
            //                         format!("{} episodes", anime.episode_count()),
            //                         true,
            //                     ),
            //                     (
            //                         "Episode Length",
            //                         format!("{} minutes", anime.episode_length()),
            //                         true,
            //                     ),
            //                 ])
            //                 .fields(vec![
            //                     ("Start Date", anime.start_date(), true),
            //                     ("End Date", anime.end_date(), true),
            //                     ("Status", anime.status(), true),
            //                 ])
            //                 .fields(vec![
            //                     ("Type", anime.subtype(), true),
            //                     (
            //                         "Popularity Rank",
            //                         &format!("#{}", anime.popularity_rank()),
            //                         true,
            //                     ),
            //                     ("Rating Rank", &format!("#{}", anime.rating_rank()), true),
            //                 ])
            //                 .field("Age Rating Guide", anime.age_rating_guide(), false)
            //                 .footer(|f| f.text("Powered by Kitsu.io"))
            //         })
            //     })
            //     .await?;
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
