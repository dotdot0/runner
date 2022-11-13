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
use rainbow_text::Rainbow;

fn main() -> std::io::Result<()>{
   //Clap initialize
   let args = Cli::parse();
   
   let config_dir = config_dir().unwrap();
   
   //Get all the user mapped commands
   let runner_path = format!("{}/runner.toml", config_dir.to_str().unwrap());
   if args.alias.is_some(){
      let map_cmd = CommandUser::new().parse_toml(runner_path.as_str());
      let alias_user = args.alias.unwrap();
      for cmd in map_cmd{
         if cmd.alias == alias_user{
            let result = Command::new(cmd.program).arg(cmd.args).output();
         }
         else{
            UserFacingError::new("Failed to run the command")
                  .reason("You have not mapped the command in runner.toml file to a alias")
                  .help("Register the alias to command in runner.toml file")
                  .print();
         }
      }
   }

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
         let rainbow = Rainbow::default();
         rainbow.write(art)?;
       
       let file = File::create(format!("{}/runner.toml", config_dir.to_str().unwrap())).unwrap();
       println!("Created runner.toml file successfully!");
   }

   }
   Ok(())
   
}
