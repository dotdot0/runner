use clap::Parser;


#[derive(Debug, Parser)]
#[clap(version, about="A cli tool that let's you map commands to a shorter alias")]
pub struct Cli{
  ///Alias Name
  pub alias: Option<String>,

  #[clap(short='i', long)]
  ///Initialize a empty runner.toml file
  pub init: bool,

  #[clap(short='m', long)]
  ///Show all the user defined mappings
  pub mapping: bool,

  #[clap(short='c', long)]
  ///Path of the the config file runner.toml
  pub config: bool,

  #[clap(short='a', long)]
  ///Map a new command to a alias right from terminal
  pub add: bool,

  #[clap(short='f', long)]
  ///Find a command mapped to the given alias
  pub find: bool
}
