use crate::components::anilist::{trending_query, Page};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

#[command]
#[only_in("guild")]
#[description("Gathers data for an anime using AniList and displays relevant information.")]
#[usage("<anime name>")]
#[example("aot season 4")]
#[example("attack on titan")]
#[example("shingeki final season 2")]
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
            let variables = trending_query::Variables {
                search: Some(String::from(anime_name)),
                amt: Some(1),
                sort: Some(vec![Some(trending_query::MediaSort::POPULARITY_DESC)]),
            };
            let page = Page::new(variables).await.unwrap();

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
