use clap::Parser;


#[derive(Debug, Parser)]
#[clap(version, about="A cli tool that let's you map commands to a shorter alias")]
pub struct Cli{
  ///Alias Name
  pub alias: Option<String>,

  #[arg(short='i', long)]
  ///Initialize a empty runner.toml file
  pub init: bool,

  #[arg(short='m', long)]
  ///Show all the user defined mappings
  pub mapping: bool,

  #[arg(short='c', long)]
  ///Path of the the config file runner.toml
  pub config: bool,

  #[arg(short='a', long)]
  ///Map a new command to a alias right from terminal
  pub add: bool,

  #[arg(short='f', long="find")]
  ///Find a command mapped to the given alias
  pub find_alias: Option<String>,

  #[arg(long = "available")]
  ///Check if a alias is available to use
  pub available_name: Option<String>
}