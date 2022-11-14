#![allow(unused)]

mod commands;
mod cli;
extern crate colorful;

use clap::Parser;
use colorful::core::StrMarker;
use commands::CommandUser;
use cli::Cli;
use dirs::config_dir;
use std::{process::Command, fs::File};
use user_error::{UserFacingError, UFE};
use rainbow_text::{Rainbow, Color};
use execute::Execute;

fn main() -> std::io::Result<()>{
   //Clap initialize
   let args = Cli::parse();
   
   let config_dir = config_dir().unwrap();
   
   //Get all the user mapped commands
   let runner_path = format!("{}/runner.toml", config_dir.to_str().unwrap());
   if args.alias.is_some(){
      //All parsed user mapped commands
      let map_cmd = CommandUser::new().parse_toml(runner_path.as_str());

      //Alias argument by user
      let alias_user = args.alias.unwrap();

      //All allias mapped by user
      let map_allias: Vec<String> = map_cmd.iter().map(|cmd| {
         cmd.alias.to_owned()
      }).collect();

      //Check if runner.toml file is empty
      if map_cmd.len() > 0{
         for cmd in map_cmd{
            let user_args: Vec<&str> = cmd.args.split(" ").map(|a| {
               a
            }).collect();
         
         //Check
         if cmd.alias.trim() == alias_user.to_str(){ 
                              let mut result = Command::new(cmd.program);
               for a in user_args{
                  result.arg(a);
               }
               let output = result.execute_output().unwrap();
               break;
            }

            //alias_user not defined
            else if !map_allias.contains(&alias_user){
               UserFacingError::new("Failed to run the command")
                     .reason("You have not mapped the command in runner.toml file to a alias")
                     .help("Register the alias to command in runner.toml file")
                     .print();
               break;
            }
         }
      }
      //Empty runner.toml file case
      else{
         UserFacingError::new("runner.toml file is empty")
               .reason("You have not mapped any command in runner.toml")
               .print();
      }
   }

   //Initialize a empty runner.toml file
   else if args.init {
       let path = config_dir.as_path().join("runner.toml");
       if path.as_path().exists(){
         UserFacingError::new("The runner.toml file already exsist")
            .reason("You have already initialized runner")
            .print()
       }
       else{
         let art = r#"
               
  ____  _  _  __ _  __ _  ____  ____ 
(  _ \/ )( \(  ( \(  ( \(  __)(  _ \
 )   /) \/ (/    //    / ) _)  )   /
(__\_)\____/\_)__)\_)__)(____)(__\_)
                             
 --------------------------------
 https://github.com/pratushrai0309/runner


         "#;
         let rainbow = Rainbow::custom(vec![Color::Red, Color::Green]);
         rainbow.write(art)?;
       
       let file = File::create(format!("{}/runner.toml", config_dir.to_str().unwrap())).unwrap();
       println!("Created runner.toml file successfully!");
   }

   }

   else if args.mapping{
      CommandUser::new().display_mapping(&runner_path)
   }
   Ok(())
   
}
