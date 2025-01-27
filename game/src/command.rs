use consume::ConsumeCommand;
use go::GoCommand;
use godmode::GodmodeCommand;
use help::HelpCommand;
use hit::HitCommand;
use look::LookCommand;
use put::PutCommand;
use quit::QuitCommand;
use search::SearchCommand;
use see::SeeCommand;
use see_player::SeePlayerCommand;
use take::TakeCommand;
use unknown::UnknownCommand;
use wait::WaitCommand;
use wear::WearCommand;

use crate::{room::Direction, Game};

mod consume;
mod go;
mod godmode;
mod help;
mod hit;
mod look;
mod put;
mod quit;
mod search;
mod see;
mod see_player;
mod take;
mod unknown;
mod wait;
mod wear;

pub(crate) enum Commands {
    Help(HelpCommand),
    Look(LookCommand),
    Search(SearchCommand),
    Go(GoCommand),
    Take(TakeCommand), // object
    Put(PutCommand),   // object
    See(SeeCommand),   // object/enemy
    SeePlayer(SeePlayerCommand),
    Hit(HitCommand),   //enemy
    Wear(WearCommand), // object weapon/armor
    Wait(WaitCommand),
    Consume(ConsumeCommand), // object consumable
    Godmode(GodmodeCommand),
    Quit(QuitCommand),
    Unknown(UnknownCommand),
}

pub(crate) trait Command {
    fn execute(&self, game: &mut Game);
}

impl Command for Commands {
    fn execute(&self, game: &mut Game) {
        match self {
            Commands::Help(cmd) => cmd.execute(game),
            Commands::Look(cmd) => cmd.execute(game),
            Commands::Search(cmd) => cmd.execute(game),
            Commands::Go(cmd) => cmd.execute(game),
            Commands::Take(cmd) => cmd.execute(game),
            Commands::Put(cmd) => cmd.execute(game),
            Commands::See(cmd) => cmd.execute(game),
            Commands::SeePlayer(cmd) => cmd.execute(game),
            Commands::Hit(cmd) => cmd.execute(game),
            Commands::Wear(cmd) => cmd.execute(game),
            Commands::Wait(cmd) => cmd.execute(game),
            Commands::Consume(cmd) => cmd.execute(game),
            Commands::Godmode(cmd) => cmd.execute(game),
            Commands::Quit(cmd) => cmd.execute(game),
            Commands::Unknown(cmd) => cmd.execute(game),
        }
    }
}

pub(crate) fn parse_input_to_command(input: &str) -> Commands {
    let args: Vec<&str> = input.split_whitespace().collect();
    match args[0] {
        "Help" => Commands::Help(HelpCommand {}),
        "Quit" => Commands::Quit(QuitCommand {}),
        "Go" => Commands::Go(GoCommand {
            direction: parse_args_to_direction(&args),
        }),
        "Look" => Commands::Look(LookCommand {}),
        "Wait" => Commands::Wait(WaitCommand {}),
        "Search" => Commands::Search(SearchCommand {}),
        "See" => Commands::See(SeeCommand {
            enemy_name: parse_args_to_item_name(&args),
        }),
        "SeePlayer" => Commands::SeePlayer(SeePlayerCommand {}),
        "Take" => Commands::Take(TakeCommand {
            item: parse_args_to_item_name(&args),
        }),
        "Put" => Commands::Put(PutCommand {
            item: parse_args_to_item_name(&args),
        }),
        "Wear" => Commands::Wear(WearCommand {
            item: parse_args_to_item_name(&args),
        }),
        "Consume" => Commands::Consume(ConsumeCommand {
            item: parse_args_to_item_name(&args),
        }),
        "Hit" => Commands::Hit(HitCommand {
            name: parse_args_to_item_name(&args),
        }),
        "Godmode" => Commands::Godmode(GodmodeCommand {}),
        _ => Commands::Unknown(UnknownCommand {}),
    }
}

pub(crate) fn parse_args_to_direction(args: &Vec<&str>) -> Direction {
    if args.len() != 2 {
        Direction::None
    } else {
        Direction::from_str(args[1])
    }
}

pub(crate) fn parse_args_to_item_name(args: &Vec<&str>) -> String {
    if args.len() < 2 {
        "To little commands arguments".to_string()
    } else {
        args[1..].join(" ")
    }
}
