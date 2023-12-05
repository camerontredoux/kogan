use crate::types::openai::{OAIReqMessage, OAIRequest, OAIResponse};
use reqwest::Client;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
#[only_in("guild")]
#[description("Translate a message from one language to another.")]
async fn translate(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let openai_token = std::env::var("OPENAI_TOKEN").unwrap();
    let uri = "https://api.openai.com/v1/chat/completions";
    let bearer = format!("Bearer {}", openai_token);

    let mut mutable_args = args.clone();

    let from: String = mutable_args.single().unwrap();
    let to: String = mutable_args.single().unwrap();

    let sentence = mutable_args.remains().unwrap();

    let prompt = format!(
        "Translate the following sentence from the language {} to {}: \"{}\"",
        from, to, sentence
    );

    let system_preamble = "You are ChatGPT, a large language model trained by OpenAI, based on the GPT-3.5 architecture.";
    let assistant_prompt =
        "You will perform the duty of a translator from one language to another.";
    let preamble = "Translate the following message accurately, but only provide a translation without any additional explanation.";

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
                content: assistant_prompt.to_string(),
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

    let from = capitalize(from);
    let to = capitalize(to);

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Translating from {} to {}", from, to))
                    .field("Original", sentence, false)
                    .field(
                        "Translation",
                        res.choices[0].message.content.to_string(),
                        false,
                    )
            })
        })
        .await?;

    Ok(())
}

fn capitalize(word: String) -> String {
    let word = word.to_lowercase();
    let mut word = word.chars();
    match word.next() {
        Some(char) => char.to_uppercase().collect::<String>() + word.as_str(),
        None => String::new(),
    }
}
