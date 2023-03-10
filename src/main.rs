mod token;
mod commands;


//use tokio;
use serenity::{
    prelude::*,
    model::prelude::*,
    Client, 
};
use serenity::model::application::command::Command;
//use serenity::http::client::Http::delete_global_application_command;

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
                None => println!("could not parse string from message contents"),
            };
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "joke" => commands::joke::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let unwanted_commands:Vec<u64> = vec![];
        for command_id in unwanted_commands{
            match ctx.http.delete_global_application_command(command_id).await{
                Ok(_) => println!("successfully deleted global application command\n"),
                Err(_) => println!("unable to delete global application command\n"),
            }
        }

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::joke::register(command)
        })
        .await;

        println!("I created the following global slash command: {:#?}", guild_command);
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

