#![allow(unused)]

mod commands;
mod cli;

use clap::Parser;
use commands::Command;
use cli::Cli;
use dirs::config_dir;

fn main() {
   // let command = Command::new();
   // command.parse_toml("/home/pratush/Documents/cli-unnamed/src/test.toml");
   // let args = Cli::parse();
   // println!("{}", args.alias);
   // let config_path = config_dir().unwrap();
   // println!("{}", config_path.to_str().unwrap())
   let map_cmd = Command::new().parse_toml("/home/pratush/Documents/cli-unnamed/src/test.toml");
   for cmd in map_cmd{
      cmd.display_mapping()
   }
}
