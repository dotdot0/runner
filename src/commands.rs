use toml::Value;
use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
struct Command{
  alias: String,
  program: String,
  args: String
}

impl Command{
  fn parse_toml(&self, path: &str){
    let mut commands: Vec<Command> = Vec::new();
  }
}