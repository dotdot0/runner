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
  pub args: Vec<String>
}

impl CommandUser{
  pub fn new() -> Self{
    Self{
      alias: String::new(),
      program: String::new(),
      args: Vec::new()
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
    UserFacingError::new("runner.toml file not found")
        .reason("You have not initialized runner or have deleted the runner.toml file")
        .help("Run runner --init to create a runner.toml file")
        .print();
  }

  commands
}

  pub fn display_mapping(&self, path: &str){
    let path_exist = Path::new(path).exists();

    // if path_exist
    self.parse_toml(path).into_iter().for_each(|cmd|{
       println!("<ALIAS_NAME> : {} -> <COMMAND_MAPPED> : {} {}", cmd.alias, cmd.program, cmd.args.join(" "))
    })
    // }
    // else{
    //     UserFacingError::new("runner.toml file don't exist in path by mapping")
    //       .reason("You have not initialized runner")
    //       .help("Run runner --init to create a runner.toml file")
    //       .print();
    //   }
  }

  pub fn find_mapping(&self, path: &str, alias_name: String) -> Result<String, UserFacingError>{

    let runner_path = Path::new(path);
    let mut find_result = String::new();
    let mut alias_vec: Vec<String> = Vec::new();
    
    if runner_path.exists(){
      let user_commands = self.parse_toml(path);

      alias_vec = user_commands.iter().map(|cmd| {
        cmd.alias.to_owned()
      }).collect();

      user_commands.into_iter().for_each(|cmd|{
        if cmd.alias == alias_name{
          find_result = format!("Command mapped to alias [{alias_name}]: [{} {}]", cmd.program, cmd.args.join(" "));
        }
        else if !alias_vec.contains(&alias_name){
          UserFacingError::new(format!("No command found mapped to the alias {alias_name}"))
            .print();
            return;
        }
      })
    }
    else{
      return Err(UserFacingError::new("runner.toml file not found")
          .reason("You have not initialized runner or have deleted the runner.toml file")
          .help("Run runner --init to create a empty runner.toml file"));
    }

    Ok(find_result) 
    } 
  
  pub fn avialable_alais(&self, path: &str, alias_name: String) -> bool{
    let alias: Vec<String> = self.parse_toml(path).iter().map(|a|{
      a.alias.to_owned()
    }).collect();
    if alias.contains(&alias_name){
      return false;
    }
    true
  }
  } 

  