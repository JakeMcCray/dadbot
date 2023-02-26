use std::fs;
use std::str::Lines;
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
        Ok(jokes) => jokes.choose(&mut rng).unwrap_or(&"Cannot select dad joke".to_string()).to_string(),

        Err(_) => "Cannot load dad joke".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("joke").description("dad tells you a joke")
}

fn load_dadjokes() -> Result<Vec<String>,()>{
    let path = Path::new("../dadjokes.txt");
    //let display = path.display();

    let jokes = match fs::read_to_string(path){
        Ok(jokes) => Ok(lines_to_strings(jokes.lines())),
        Err(_) => Err(()),
    };
    jokes
}

fn lines_to_strings(lines: Lines) -> Vec<String>{
    let mut strings = Vec::new();
    for line in lines{
        strings.push(line.to_string());
    }
    strings
}
