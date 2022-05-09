use serenity::{
    client::Context, framework::standard::macros::hook, model::channel::Message, utils::Color,
};

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message) {
    msg.channel_id.say(&ctx.http, "message delayed").await.ok();
}

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, command_name: &str) {
    if let Err(err) = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Unkown command")
                    .description(&format!(
                        "Command {} does not exist. Type .help for a list of available commands.",
                        command_name
                    ))
                    .color(Color::RED)
            })
        })
        .await
    {
        println!("Error, {}", err);
    }
}
