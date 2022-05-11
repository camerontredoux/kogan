use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{
        channel::{ChannelType, Message, PermissionOverwrite, PermissionOverwriteType},
        id::{GuildId, RoleId},
        Permissions,
    },
    utils::Color,
};

#[command]
#[description = "Creates an announcement channel if it doesn't exit, and sends a message to all users in a specific group."]
#[allowed_roles("ANNOUNCEMENTS")]
async fn announce(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let channels = guild_id.channels(&ctx.http).await?;

    if let Some(_) = channels.values().find(|c| c.name() == "announcements") {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.color(Color::RED)
                        .title("Server creation failed!")
                        .description("Server already exists.")
                })
            })
            .await?;
    } else {
        let _ = guild_id
            .create_channel(&ctx.http, |c| {
                c.kind(ChannelType::Text)
                    .name("announcements")
                    .topic("Kōgan - Anime announcements and countdowns!")
                    .position(1)
                    .permissions(vec![PermissionOverwrite {
                        allow: Permissions::VIEW_CHANNEL,
                        deny: Permissions::SEND_MESSAGES | Permissions::SEND_TTS_MESSAGES,
                        kind: PermissionOverwriteType::Role(RoleId(973450438353506354)),
                    }])
                    .category(972981323675361290)
            })
            .await?;
    }

    Ok(())
}
