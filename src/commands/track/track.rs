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
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(Color::DARK_GREEN)
                            .title(format!("Tracking {}", anime_name))
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
