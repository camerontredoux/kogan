use std::time::Duration;

use serde_json::json;
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

use crate::commands::anime::Anime;

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

const QUERY: &str = "
query AnimeData {
    Page(page: 1, perPage: 10) {
      media(type: ANIME, sort: TRENDING_DESC) {
        id
        idMal
        title {
          romaji
          english
        }
        description
        coverImage {
          medium
        }
        averageScore
        meanScore
        format
        nextAiringEpisode {
          id
        }
        status
        startDate {
          year
          month
          day
        }
        endDate {
          year
          month
          day
        }
        episodes
        duration
        seasonYear
        season
      }
    }
  }
  
  
";

#[command]
#[description("Shows the top trending anime using the kitsu.io API.")]
pub async fn trending(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let graphql_json = json!({
        "query": QUERY,
    });

    let graphql = client
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(graphql_json.to_string())
        .send()
        .await?
        .text()
        .await?;

    let result = serde_json::from_str::<serde_json::Value>(&graphql).unwrap();
    println!("{:#?}", result);

    let anime_json = reqwest::get("https://kitsu.io/api/edge/trending/anime")
        .await?
        .json::<serde_json::Value>()
        .await?;

    let mut index = 0;

    let anime_list = anime_json["data"].as_array().unwrap();

    let m = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Top trending anime of the week.")
                    .description("This is a list of the top trending anime. Click next or previous to cycle through your options!")
                    .color(Color::GOLD)
                    .footer(|f| f.text("List is from kitsu.io, updated weekly."))
            })
            .content("_List will dissapear after **one** minute!_")
            .components(|c| c.add_action_row(action_row()))
        })
        .await?;

    let mut cic = m
        .await_component_interactions(&ctx)
        .timeout(Duration::from_secs(60))
        .build();

    while let Some(mci) = cic.next().await {
        match mci.data.custom_id.as_str() {
            "next" => {
                index += 1;
                if index >= anime_list.len() {
                    index = 0;
                }
            }
            "previous" => {
                if index == 0 {
                    index = anime_list.len() - 1;
                } else {
                    index -= 1;
                }
            }
            _ => continue,
        }
        let anime = Anime::new(&anime_json, index);
        mci.create_interaction_response(&ctx.http, |r| {
            r.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.embed(|e| {
                        e.title(format!(
                            "Info for {} (Rated {})",
                            anime.name, anime.age_rating
                        ))
                        .color(Color::GOLD)
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
