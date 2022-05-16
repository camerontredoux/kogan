use serenity::{
    client::Context, framework::standard::macros::hook, model::channel::Message, utils::Color,
};

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message) {
    msg.channel_id.say(&ctx.http, "Message delayed").await.ok();
}

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, command_name: &str) {
    msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Unkown command")
                    .description(&format!(
                        "Command `{}` does not exist. Type `.help` for a list of available commands.",
                        command_name
                    ))
                    .color(Color::RED)
            })
        })
        .await.ok();
}
