use crate::{
    components::anilist::{trending_query, Page},
    services::init_services,
    types::openai::{OAIReqMessage, OAIRequest, OAIResponse},
};
use reqwest::Client;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

#[command]
#[only_in("guild")]
#[description("Recommends a list of anime to watch based on your favorited animes.")]
async fn recommend(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
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

    let user_service = init_services().await.user_service;

    let anime_name = args.remains();
    let user = user_service.get_user(msg.author.id.0.to_string()).await;
    match user {
        Ok(u) => match u {
            Some(user) => {
                let openai_token = std::env::var("OPENAI_TOKEN").unwrap();
                let uri = "https://api.openai.com/v1/chat/completions";
                let bearer = format!("Bearer {}", openai_token);

                let prompt = format!(
                    "What are the top 10 anime you would recommend if I watch \"{}\"?",
                    user.animes
                        .clone()
                        .into_iter()
                        .take(10)
                        .collect::<Vec<String>>()
                        .join(", ")
                );

                let system_preamble = "You are ChatGPT, a large language model trained by OpenAI, based on the GPT-3.5 architecture. You are also an assistant that provides recommendations on anime to watch according to a user's list of anime they already enjoy watching.";
                let preamble = "Answer the following question accurately, but only provide an ordered list without any introductory paragraph. The answer should simply be a list of responses.";

                let oai_request = OAIRequest {
                    messages: vec![
                        OAIReqMessage {
                            content: format!("{} {}", preamble, prompt),
                            role: "user".to_string(),
                        },
                        OAIReqMessage {
                            content: system_preamble.to_string(),
                            role: "system".to_string(),
                        },
                        OAIReqMessage {
                            content: format!(
                                "List of anime I enjoy: {}",
                                user.animes
                                    .into_iter()
                                    .take(10)
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            ),
                            role: "assistant".to_string(),
                        },
                    ],
                    model: "gpt-3.5-turbo".to_string(),
                };

                let client = Client::new();
                let request = client
                    .post(uri)
                    .header("Content-Type", "application/json")
                    .header("Authorization", bearer)
                    .json(&oai_request);
                msg.channel_id.broadcast_typing(&ctx.http).await?;
                let res: OAIResponse = request.send().await?.json().await?;
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.color(Color::RED)
                                .title("Recommendations")
                                .field("Prompt", prompt, false)
                                .field("Response", res.choices[0].message.content.clone(), false)
                        })
                    })
                    .await?;
            }
            None => {}
        },
        Err(_) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.color(Color::RED).title("No user found in database")
                            .description("Have you set any favorite animes yet? Do this first before trying to recommend anime to watch.")
                        })
                    }).await?;
        }
    }

    Ok(())
}
