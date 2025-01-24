
use help::HelpCommand;
use look::LookCommand;
use search::SearchCommand;
use go::GoCommand;
use take::TakeCommand;
use put::PutCommand;
use see::SeeCommand;
use see_player::SeePlayerCommand;
use hit::HitCommand;
use wear::WearCommand;
use consume::ConsumeCommand;
use godmode::GodmodeCommand;
use unknown::UnknownCommand;
use wait::WaitCommand;
use quit::QuitCommand;

mod help;
mod look;
mod search;
mod go;
mod take;
mod put;
mod see;
mod see_player;
mod hit;
mod wear;
mod wait;
mod consume;
mod godmode;
mod quit;
mod unknown;

enum Commands {
    Help(HelpCommand),
    Look(LookCommand),
    Search(SearchCommand),
    Go(GoCommand),
    Take(TakeCommand), // object
    Put(PutCommand), // object
    See(SeeCommand), // object/enemy
    SeePlayer(SeePlayerCommand), 
    Hit(HitCommand), //enemy
    Wear(WearCommand), // object weapon/armor
    Wait(WaitCommand),
    Consume(ConsumeCommand), // object consumable
    Godmode(GodmodeCommand),
    Quit(QuitCommand),
    Unknown(UnknownCommand),
}

pub trait Command {
    fn execute(&self);    
}

impl Command for Commands {
    fn execute(&self) {
        match self {
            Commands::Help(cmd) => cmd.execute(),
            Commands::Look(cmd) => cmd.execute(),
            Commands::Search(cmd) => cmd.execute(),
            Commands::Go(cmd) => cmd.execute(),
            Commands::Take(cmd) => cmd.execute(),
            Commands::Put(cmd) => cmd.execute(),
            Commands::See(cmd) => cmd.execute(),
            Commands::SeePlayer(cmd) => cmd.execute(),
            Commands::Hit(cmd) => cmd.execute(),
            Commands::Wear(cmd) => cmd.execute(),
            Commands::Wait(cmd) => cmd.execute(),
            Commands::Consume(cmd) => cmd.execute(),
            Commands::Godmode(cmd) => cmd.execute(),
            Commands::Quit(cmd) => cmd.execute(),
            Commands::Unknown(cmd) => cmd.execute(),
        }
    }
}

pub fn parse_input_to_command(input: &str) -> Commands {
    let args: Vec<&str> = input.split_whitespace().collect();
    match args[0] {
        "Help" => Commands::Help(HelpCommand{}),
        "Quit" => Commands::Quit(QuitCommand{}),
        "Go" => Commands::Go(GoCommand{}),
        _ => Commands::Unknown(UnknownCommand{}),
    }
}
