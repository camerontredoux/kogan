use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{
        channel::{ChannelType, Message},
        id::GuildId,
    },
    utils::Color,
};

#[command]
#[description = "Creates an announcement channel if it doesn't exit, and sends a message to all users in a specific group."]
#[owners_only]
async fn announce(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| m.content("message"))
        .await
        .unwrap();

    let channels = GuildId(972981323130093648)
        .channels(&ctx.http)
        .await
        .unwrap();

    if let Some(_) = channels.values().find(|c| c.name() == "announcements") {
        if let Err(err) = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.color(Color::RED)
                        .title("Server creation failed!")
                        .description("Server already exists.")
                })
            })
            .await
        {
            println!("Error sending command: {}", err);
        }
    } else {
        let _ = GuildId(972981323130093648)
            .create_channel(&ctx.http, |c| {
                c.kind(ChannelType::Text)
                    .name("Announcements")
                    .topic("Server announcements")
                    .category(972981323675361290)
            })
            .await;
    }

    Ok(())
}
