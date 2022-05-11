use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Color,
};

#[command]
async fn track(ctx: &Context, msg: &Message) -> CommandResult {
    match msg.channel_id.name(&ctx).await {
        Some(m) => {
            if m.as_str() != "announcements" {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.description(
                                "This command must be used in the `announcements` channel",
                            )
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
