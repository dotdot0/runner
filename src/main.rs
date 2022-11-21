#![allow(unused)]

//Modules
mod commands_parse;
mod cli;
mod add;

//std
use std::{process::Command, fs::File};

//External crates
use clap::Parser;
use colorful::{Color, Colorful};
use commands_parse::CommandUser;
use cli::Cli;
use dirs::config_dir;
use user_error::{UserFacingError, UFE};
use execute::Execute;

fn main() -> std::io::Result<()>{
   //Clap initialize
   let args = Cli::parse();
   
   let config_dir = config_dir().unwrap();
   
   //Get all the user mapped commands
   let runner_path = format!("{}/runner.toml", config_dir.to_str().unwrap());
   if args.alias.is_some() && std::env::args().nth(2).is_none(){
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
         
            if cmd.alias.trim() == alias_user.as_str(){ 
               let mut result = Command::new(cmd.program);
               for a in cmd.args{
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
                             
  ________________________________________
: https://github.com/pratushrai0309/runner :
 __________________________________________

         "#;
      println!("{}", art.gradient(Color::Green).bold()); 
      let file = File::create(format!("{}/runner.toml", config_dir.to_str().unwrap())).unwrap();
      println!("Created runner.toml file successfully!");
   }

   }

   else if args.mapping{
      CommandUser::new().display_mapping(&runner_path)
   }

   else if args.config{
      let path = config_dir.join("runner.toml");
      if path.exists(){
         println!("The path to your runner.toml file: {:#?}", path.as_path());
      }
      else{
         UserFacingError::new("runner.toml file not found")
               .reason("You have not initailized runner")
               .help("Run runner --init to create a runner.toml file")
               .print();
      }
   }

   else if args.add{
      add::add(&runner_path);
   }

   else if args.find{
      let mut find_args = std::env::args();
      if find_args.len() == 3{
         let alias_name = find_args.nth(2).unwrap(); 
         match CommandUser::new().find_mapping(&runner_path, alias_name){
            Ok(cmd) => println!("{cmd}"),
            Err(e) => e.print()
         }
      }
      else{
         UserFacingError::new("--find command accepts a <ALIAS_NAME> but none was provided")
         .print()
      }
   }
   Ok(())
   
}
