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
#[description("Shows the top trending anime using the kitsu.io API.")]
pub async fn trending(ctx: &Context, msg: &Message) -> CommandResult {
    let anime_json = reqwest::get("https://kitsu.io/api/edge/trending/anime")
        .await?
        .json::<serde_json::Value>()
        .await?;

    let anime_list = anime_json["data"].as_array().unwrap();

    let mut index = 0;
    let mut name = anime_list[index]["attributes"]["canonicalTitle"]
        .as_str()
        .unwrap();

    let m = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Top trending anime")
                    .description("This is a list of the top trending anime.")
                    .color(Color::DARK_GREEN)
                    .field("Name", name, true)
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
                name = anime_list[index]["attributes"]["canonicalTitle"]
                    .as_str()
                    .unwrap();
            }
            "previous" => {
                if index == 0 {
                    index = anime_list.len() - 1;
                } else {
                    index -= 1;
                }
                name = anime_list[index]["attributes"]["canonicalTitle"]
                    .as_str()
                    .unwrap();
            }
            _ => continue,
        }
        mci.create_interaction_response(&ctx.http, |r| {
            r.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.embed(|e| {
                        e.title("Top trending anime")
                            .description("This is a list of the top trending anime.")
                            .color(Color::DARK_GREEN)
                            .field("Name", name, true)
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
