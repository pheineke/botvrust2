mod modules;

use serenity::{
    async_trait,
    //prelude::*,
    client::{Client, Context, EventHandler},
    model::{channel::Message, gateway::{GatewayIntents, Ready}},
    framework::standard::{
            macros::{command, group},
            {StandardFramework, Configuration, CommandResult}
        }
};
use dotenv::dotenv;
use std::env;

//MODULES//////////
use crate::modules::modul0::*;


#[group]
#[commands(ping,multiply)]
struct General;



struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Hier können Sie auf eingehende Nachrichten reagieren
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN muss gesetzt sein");
    //////

    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("$")); // set the bot's prefix to "~"

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}