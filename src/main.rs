mod token;
//use tokio;
use serenity::{
    prelude::*,
    model::prelude::*,
    Client, 
};

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message){
        if msg.author.bot{
            return;
        }
        if msg.content.contains("im "){
            let name = msg.content.split("im ").last();
            match name{
                Some(name) => if let Err(why) = msg.channel_id.say(ctx.http,format!("hi {} im dad",name)).await{
                    println!("could not sent dad quip because {}",why);
                },
                None => println!("could not parse string"),
            };
        }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = token::TOKEN;
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    println!("starting dadbot rust version");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

