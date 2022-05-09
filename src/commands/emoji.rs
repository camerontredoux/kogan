use serenity::{
    client::Context,
    framework::standard::{buckets::RevertBucket, macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
#[aliases("kitty", "neko")]
#[bucket = "emoji"]
async fn cat(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, ":cat:").await?;

    if let Some(saying) = args.current() {
        msg.channel_id
            .say(&ctx.http, &format!("args: {}", saying))
            .await?;
    }

    Err(RevertBucket.into())
}
