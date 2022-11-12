#![allow(unused)]

mod commands;

use toml::Value;
use serde_derive::Deserialize;

use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
struct Command{
    alias: String,
    program: String,
    args: String
}

fn main() {
    let mut commands: Vec<Command> = Vec::new();
    let mut command: Option<Command> = None;
    let mut file = File::open("/home/pratush/Documents/cli-unnamed/src/test.toml").expect("Unable to find file");
    let mut buf: String = String::new();
    file.read_to_string(&mut buf).unwrap();
    let main_data:Vec<String> = buf.split("[Command]").map(|s|{
        s.trim().to_owned()
    }).collect();
    //println!("{:#?}", main_data);
    for i in main_data{
        if i  == ""{
            continue;
        }
        else{
           let command :Option<Command> = Some(toml::from_str(&i).unwrap()); 
           commands.push(command.unwrap()) 
        }
    }
    println!("{:#?}", commands);
    // // println!("{:#?}", main_data);
    // let command: Command = toml::from_str(r#"
    // alias = "gitadd"
    // program = "git"
    // args = "add ."
    // "#).expect("Unable to parse");
    // println!("{:#?}", command);
}

#[test]
fn it_works(){
    let command: Command = toml::from_str(r#"
    alias = "c"
    program = "code"
    args = "."
    "#).expect("Unable to parse");

    assert_eq!(command.alias, "c");
    assert_eq!(command.program, "code");
    assert_eq!(command.args, ".")
}
