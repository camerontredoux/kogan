use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Color,
};

#[command("about")]
#[sub_commands(bot)]
#[description = "Send's the server and bot rules as a DM to the user."]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.author
        .dm(&ctx.http, |m| {
            m.content(
                "This is a test DM for the server's rules. Will become an embed in the future.",
            )
        })
        .await?;

    Ok(())
}

#[command]
#[description = "Send's the bot rules as a DM to the user"]
async fn bot(ctx: &Context, msg: &Message) -> CommandResult {
    msg.
    author.dm(&ctx.http, |m| {
        m.embed(|e| {
                e.color(Color::ORANGE)
                    .title("Kōgan Bot Rules")
                    .description("Rules pertaining to the bot usage")
                    .fields(vec![
                        (
                            "Rule 1",
                            "Do not spam bot commands. I will be forced to implement a rate limiter, and I do not want to do that. Spamming commands will cause the bot to be blacklisted by the Discord API, and could cost me money depending on the host I use.",
                            false,
                        ),
                        (
                            "Rule 2",
                            "Uhhh no other rules, enjoy the bot.",
                            false
                        )
                    ])
                    .footer(|f| {
                        f.text("Created by Cameron Tredoux. Used on TLMBZ's server.")
                    })
            })
        })
        .await?;

    Ok(())
}
