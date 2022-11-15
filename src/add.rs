//std
use std::fs::File;

//External crates
use inquire::Text;

use std::path::Path; 

pub fn add(path: &str) -> std::io::Result<()>{
  let alias = Text::new("Alias: ").with_help_message("Alias").prompt().unwrap();
  let program = Text::new("Program: ").with_help_message("Program").prompt().unwrap();
  let args = Text::new("Args: ").with_help_message("args").prompt().unwrap();

  let runner_path = Path::new(path);

  if runner_path.exists(){

  }

  println!("
  [Command]
  alias = {alias}
  program = {program}
  args = {args}
  ");
  Ok(())
}