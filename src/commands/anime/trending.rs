use std::time::Duration;

use serenity::{
    builder::{CreateActionRow, CreateButton},
    client::Context,
    framework::standard::{macros::command, CommandResult},
    futures::StreamExt,
    model::{
        channel::Message,
        interactions::{message_component::ButtonStyle, InteractionResponseType},
    },
    utils::Color,
};

use crate::components::anilist::{trending_query, Page};

fn next() -> CreateButton {
    let mut button = CreateButton::default();
    button.label("Next");
    button.style(ButtonStyle::Success);
    button.custom_id("next");
    button
}
fn previous() -> CreateButton {
    let mut button = CreateButton::default();
    button.label("Previous");
    button.style(ButtonStyle::Success);
    button.custom_id("previous");
    button
}

fn action_row() -> CreateActionRow {
    let mut ar = CreateActionRow::default();
    ar.add_button(previous());
    ar.add_button(next());
    ar
}

#[command]
#[description("Shows the top trending anime using the AniList API.")]
pub async fn trending(ctx: &Context, msg: &Message) -> CommandResult {
    let variables = trending_query::Variables {
        amt: Some(10),
        search: None,
        sort: Some(vec![Some(trending_query::MediaSort::TRENDING_DESC)]),
    };
    let trending = Page::new(variables).await.unwrap();

    let mut index = 0;

    let m = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Top trending anime of the week.")
                    .description("This is a list of the top trending anime. Click next or previous to cycle through your options!")
                    .color(Color::GOLD)
                    .footer(|f| f.text("List is from AniList, updated weekly."))
            })
            .content("_List will dissapear after **one** minute!_")
            .components(|c| c.add_action_row(action_row()))
        })
        .await?;

    let mut cic = m
        .await_component_interactions(&ctx)
        .timeout(Duration::from_secs(120))
        .build();

    while let Some(mci) = cic.next().await {
        match mci.data.custom_id.as_str() {
            "next" => {
                index += 1;
                if index >= trending.media.len() {
                    index = 0;
                }
            }
            "previous" => {
                if index == 0 {
                    index = trending.media.len() - 1;
                } else {
                    index -= 1;
                }
            }
            _ => continue,
        }
        let media = trending.media.get(index).unwrap();
        mci.create_interaction_response(&ctx.http, |r| {
            r.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.embed(|e| {

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
        })
        .await?
    }

    m.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| e.description("_Selection timed out!_").color(Color::RED))
        })
        .await?;
    m.delete(&ctx.http).await?;

    Ok(())
}
