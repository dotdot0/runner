#![allow(unused)]

mod commands;

use commands::Command;

fn main() {
   let command = Command::new();
   command.parse_toml("/home/pratush/Documents/cli-unnamed/src/test.toml") 
}
