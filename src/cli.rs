use clap::Parser;


#[derive(Debug, Parser)]
#[clap(version, about="A cli tool that let's you map commands to a shorter alias")]
pub struct Cli{
  ///Alias Name
  pub alias: String,

  #[clap(short='i', long)]
  init: bool
}