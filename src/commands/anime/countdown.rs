use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

use crate::components::anilist::{trending_query, Page};

#[command]
#[description = "Displays countdown until next episode release for the specified anime. Also displays the usual airing date."]
#[aliases(cd)]
#[only_in("guild")]
#[usage("<anime name>")]
#[example("aot season 4")]
#[example("attack on titan")]
#[example("shingeki final season 2")]
pub async fn countdown(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
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
        Some(anime) => {
            let variables = trending_query::Variables {
                amt: Some(1),
                search: Some(String::from(anime)),
                sort: Some(vec![Some(trending_query::MediaSort::POPULARITY_DESC)]),
            };
            let page = Page::new(variables).await.unwrap();

            let anime = match page.media.get(0) {
                Some(a) => a,
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
                        e.title(format!(
                            "Countdown for {} (_{}_)",
                            anime.title_romaji(),
                            anime.title_english()
                        ))
                        .color(Color::GOLD)
                        .field(
                            format!("Episode {} Airs in: ", anime.episode()),
                            anime.timeUntilAiring(),
                            true,
                        )
                        .field("Airing Time: ", anime.airingAt(), false)
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
