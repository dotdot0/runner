use std::{fs::File, io::Read};

use toml::Value;
use serde_derive::Deserialize;
use std::path::Path;
use user_error::{UserFacingError, UFE};

#[derive(Debug, Deserialize)]
pub struct CommandUser{
  pub alias: String,
  pub program: String,
  pub args: String
}

impl CommandUser{
  pub fn new() -> Self{
    Self{
      alias: String::new(),
      program: String::new(),
      args: String::new()
    }
  }
  pub fn parse_toml(&self, path: &str) -> Vec<CommandUser>{
    let mut commands: Vec<CommandUser> = Vec::new();
    let mut command: Option<CommandUser> = None;
    let path_exist = Path::new(path).exists();
    if path_exist{
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
  }else{
    UserFacingError::new("runner.toml file not find")
        .reason("You have not initialized runner")
        .help("Run runner --init to create a runner.toml file")
        .print();
  }

  commands
}

  pub fn display_mapping(&self, path: &str){
    let path_exist = Path::new(path).exists();
    for cmd in self.parse_toml(path){
      if path_exist{
        println!("<ALIAS_NAME> : {} -> <COMMAND_MAPPED>: {} {}", cmd.alias, cmd.program, cmd.args)
      }
      else{
        UserFacingError::new("runner.toml file don't exist in path")
          .reason("You have not initialized runner")
          .help("Run runner --init to create a runner.toml file")
          .print();
      }
  }
  }
}