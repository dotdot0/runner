use std::{fs::File, io::Read};

use toml::Value;
use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Command{
  pub alias: String,
  pub program: String,
  pub args: String
}

impl Command{
  pub fn new() -> Self{
    Self{
      alias: String::new(),
      program: String::new(),
      args: String::new()
    }
  }
  pub fn parse_toml(&self, path: &str){
    let mut commands: Vec<Command> = Vec::new();
    let mut command: Option<Command> = None;
    let mut file = File::open(path).expect("Unable to open file");
    let mut buf: String = String::new();
    file.read_to_string(&mut buf);
    let cmd_data:Vec<String> = buf.split("[Command]").map(|x| {
      x.trim().to_owned()
    }).collect();
    for cmd in cmd_data{
      if cmd == ""{
        continue;
      }
      else{
        command = Some(toml::from_str(&cmd).unwrap());
        commands.push(command.unwrap())
      }
    }
    println!("{:#?}", commands)
  }
}