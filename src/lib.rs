use std::{collections::HashSet, process};

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::{
        standard::{
            buckets::{LimitedFor, RevertBucket},
            help_commands,
            macros::{command, group, help, hook},
            Args, CommandGroup, CommandResult, HelpOptions,
        },
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, gateway::Ready, id::UserId, interactions::Interaction},
    prelude::GatewayIntents,
    utils::Color,
    Client,
};

#[group]
#[commands(usage, rules, cat)]
struct General;

struct Handler;

#[hook]
async fn delay_action(ctx: &Context, msg: &Message) {
    msg.channel_id.say(&ctx.http, "message delayed").await.ok();
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data: Ready) {}

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {}
}

#[help]
#[individual_command_tip = "If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[strikethrough_commands_tip_in_guild("")]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
pub async fn init(token: String) {
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;
    let http = Http::new(&token);
    let bot_id = match http.get_current_application_info().await {
        Ok(_) => match http.get_current_user().await {
            Ok(bot_id) => bot_id.id,
            Err(err) => {
                println!("Failed to access bot id: {}", err);
                process::exit(1);
            }
        },
        Err(err) => {
            println!("Failed to access bot info: {}", err);
            process::exit(1);
        }
    };
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(".").with_whitespace(true).on_mention(Some(bot_id)))
        .help(&HELP)
        .bucket("emoji", |b| {
            b.delay(5)
                .delay_action(delay_action)
                .limit_for(LimitedFor::User)
        })
        .await
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    if let Err(err) = client.start().await {
        println!("Failed to start client: {}", err);
        process::exit(1);
    };
}

#[command]
async fn usage(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(err) = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.color(Color::ORANGE)
                    .title("How to use Kōgan")
                    .description("List of Kōgan commands and their usage")
                    .fields(vec![
                        (
                            "Prefix",
                            ". (period) - must prefix all commands or use the @Kogan mention",
                            false,
                        ),
                        ("Commands", "usage/help - displays this dialog box with all the commands for Kōgan\nrules - DM's the user with a list of server rules and bot rules", false),
                    ])
            })
        })
        .await
    {
        msg.reply(&ctx.http, "Error using this command").await.ok();
        println!("{}", err);
    };

    Ok(())
}

#[command]
#[description = "Send's the server and bot rules as a DM to the user."]
async fn rules(ctx: &Context, msg: &Message) -> CommandResult {
    let dm = msg
        .author
        .dm(&ctx.http, |m| {
            m.content(
                "This is a test DM for the server's rules. Will become an embed in the future.",
            )
        })
        .await;

    if let Err(err) = dm {
        println!("Error DMing the author {}", err);
    }

    Ok(())
}

#[command]
#[aliases("kitty", "neko")]
#[bucket = "emoji"]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, ":cat:").await?;

    Err(RevertBucket.into())
}
