#![allow(dead_code)]
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod command_store;
mod json_storage;

use command_store::CommandStore;
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use std::process::{Command, exit};
use std::collections::HashMap;
use std::env;
use serde_json::{Value, Error};

fn main() {
    let args: Vec<String> = env::args().collect();

    let maybe_sub_command = args.get(1);
    if let Some(sub_command) = maybe_sub_command {
        let (_, sub_args) = args.split_at(2);
        match sub_command.as_ref() {
            "run" => run(sub_args),
            "ls" => ls(sub_args),
            _ => print_usage()
        }
    } else {
        print_usage();
    }
}

fn ls(args: &[String]) {
    let command_store = CommandStore::new("data/command.json");

    match args.len() {
        0 => println!("show all"),
        1 => println!("show grep"),
        _ => println!("too match args")
    }
}

fn run(args: &[String]) {
    let index_str = match args.first() {
        Some(x) => x,
        None => {
            println!("too few args");
            exit(0);
        }
    };
    let index: usize = match index_str.parse() {
        Ok(x) => x,
        Err(_) => {
            println!("index must be number");
            exit(0);
        }
    };

    let commands: Vec<Value> = read_commands();
    let maybe_command = commands.into_iter().find(|command| command["index"] == index);
    if let Some(command) = maybe_command {
        let func: &str = command["func"].as_str().unwrap();
        exec_func(func, args);
    } else {
        println!("Index {} is not found", index);
    }
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

fn read_commands() -> Vec<Value> {
    let mut file = File::open("data/commands.json").unwrap();
    let mut commands_json_string = String::new();
    file.read_to_string(&mut commands_json_string);

    let value: Value = serde_json::from_str(&commands_json_string).unwrap();

    let commands = match value {
        Value::Array(x) => x,
        _ => vec![]
    };
    return commands;
}

fn print_usage() {
    println!("Usage: ...");
}
