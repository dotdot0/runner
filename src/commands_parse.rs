//Parses the runner.toml file and return a vector of user mapped commands
/*
Using toml, serde to parse the toml file
[Command]
alias = "alias"
program = "program"
args = "args"
are deserialized to CommandUser struct
*/

use std::{fs::File, io::Read};

use clap::Error;
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
    //Vec of user commands
    let mut commands: Vec<CommandUser> = Vec::new();
    let mut command: Option<CommandUser> = None;
    let path_exist = Path::new(path).exists();
    
    //Only parse the toml file if it exist else custom error 
    if path_exist{
      let mut file = File::open(path).expect("Unable to open file");
      let mut buf: String = String::new();
      file.read_to_string(&mut buf);

      //Getting the data
      let cmd_data:Vec<String> = buf.split("[Command]").map(|x| {
        x.trim().to_owned()
      }).collect();

      //Iter through the vector and parse each command
      for (i, cmd) in cmd_data.iter().enumerate(){
        if cmd == ""{
          continue;
        }
        else{
          match toml::from_str(&cmd) {
              Err(e) => {
                UserFacingError::new("Error in runner.toml file")
                .reason(format!("There is a error in runner.toml in command no: {}", i))
                .help(r#"The command should be written in this format with all field mandatory:
    [Command]
    alias = "alais"
    program = "program"
    args = "args"#)
                .print();
                break;
              },
              Ok(t) => {
                command = Some(t);
                commands.push(command.unwrap())
              }
          }
          
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