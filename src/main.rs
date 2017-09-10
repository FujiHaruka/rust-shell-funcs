#![allow(dead_code)]
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod command_manager;
mod json_storage;

use command_manager::CommandManager;
use std::process::{Command};
use std::env;

const SUB_COMMAND_RUN: &str = "run";
const SUB_COMMAND_SHOW: &str = "ls";

const STORAGE_PATH: &str = "data/commands.json";

fn main() {
    let args: Vec<String> = env::args().collect();

    let maybe_sub_command = args.get(1);
    if let Some(sub_command) = maybe_sub_command {
        let (_, sub_args) = args.split_at(2);
        match sub_command.as_ref() {
            SUB_COMMAND_RUN => {
                run(sub_args).map_err(|e| {
                    println!("{}", e)
                }).unwrap();
            },
            SUB_COMMAND_SHOW => {
                ls(sub_args).map_err(|e| {
                    println!("{}", e)
                }).unwrap();
            },
            _ => print_usage()
        };
    } else {
        print_usage();
    }
}

fn ls(args: &[String]) -> Result<(), String> {
    let mut command_store = CommandManager::new(STORAGE_PATH);

    let search_word = match args.len() {
        0 => "",
        1 => &args[0],
        _ => return Err("too match args".to_string())
    };
    command_store.update_indexes_by(search_word).unwrap();
    command_store.show_commands();
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    let index_str = match args.first() {
        Some(index) => index,
        None => return Err("too few args".to_string()),
    };
    let index: usize = match index_str.parse() {
        Ok(x) => x,
        Err(_) => return Err("index must be number".to_string()),
    };

    let command_store = CommandManager::new(STORAGE_PATH);
    let maybe_command = command_store.get_command(index);
    if let Some(command) = maybe_command {
        let func = command.func;
        exec_func(&func, args);
    } else {
        println!("Index {} is not found", index);
    }
    Ok(())
}

fn exec_func (func_str: &str, args: &[String]) {
    let mut command = func_str.to_string();
    for i in 1..9 {
        let var_name = format!("${}", i);
        let var_value = args.get(i);
        if let Some(value) = var_value {
            command = command.replace(&var_name, &value).to_string();
        }
    }
    println!("> {}", command);
    Command::new("sh").arg("-c").arg(command).spawn().expect("failed");
}

fn print_usage() {
    println!("Usage: ...");
}
