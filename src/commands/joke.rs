use std::fs;
use std::path::Path;
use rand::thread_rng;

use rand::seq::SliceRandom;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

//TODO this is the function that will return a vector of dadjokes
//I will need to make a "joke" class before i finish this function


pub fn run(_options: &[CommandDataOption]) -> String{
    let jokes: Result<Vec<String>,()> = load_dadjokes();
    
    let mut rng = thread_rng();

    match jokes{
        Ok(jokes) => jokes.choose(&mut rng).unwrap_or(&"Hmm, I can't decide from all of my funny jokes".to_string()).to_string(),

        Err(_) => "Sorry sport, I don't have any jokes to tell right now".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("joke").description("dad tells you a joke")
}

fn load_dadjokes() -> Result<Vec<String>,()>{
    let path = Path::new("/home/visedsquirrel/projects/rust/dadbot/dadjokes.txt");
    //let display = path.display();

    let jokes = match fs::read_to_string(path){
        Ok(jokes) => Ok(jokes.lines().map(|x| x.replace("<>","\n")).collect()),
        Err(_) => Err(()),
    };
    jokes
}
