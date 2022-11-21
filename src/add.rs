//std
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path; 

//External crates
use inquire::Text;
use user_error::{UserFacingError, UFE};

//project files
use crate::commands_parse::CommandUser;

pub fn add(path: &str) -> std::io::Result<()>{
  let alias = Text::new("Alias:").with_help_message("Alias").prompt().unwrap().trim().to_owned();
  let program = Text::new("Program:").with_help_message("cli Program").prompt().unwrap().trim().to_owned();
  let args: Vec<String> = Text::new("Args:").with_help_message("like arg1, arg2")
  .prompt()
  .unwrap().split(",").map(|a| {
    a.trim().to_owned()
  }).collect();

  let buf = format!("
[Command]
alias = \"{alias}\"
program = \"{program}\"
args = {:?}
  ", args);

  let runner_path = Path::new(path);

  if runner_path.exists(){
    let map_alias: Vec<String> = CommandUser::new().parse_toml(path).iter().map(|cmd| {
      cmd.alias.to_owned()
    }).collect();
    if !map_alias.contains(&alias){
      let mut file = OpenOptions::new().append(true).open(path).unwrap();
      file.write(buf.as_bytes()).unwrap();
      println!("Alias: {alias} added to runner.toml!")
    }
    else{
      UserFacingError::new("Alias already exist")
          .reason("You have already mapped the alias to a commnad")
          .print()
    }
  }
  else{
    UserFacingError::new("runner.toml file not found")
      .reason("You have not initialized runner")
      .help("Run runner --init to create a runner.toml file")
      .print();
  }


  Ok(())
}