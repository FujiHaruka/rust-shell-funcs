extern crate serde_json;

use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;
use serde_json::{Value, Error};

fn main() {
    let mut file = File::open("data/commands.json").unwrap();
    let mut commands_json_string = String::new();
    file.read_to_string(&mut commands_json_string);

    let value: Value = serde_json::from_str(&commands_json_string).unwrap();

    let commands = match value {
        Value::Array(x) => x,
        _ => vec![]
    };


    let command = commands.get(0).unwrap();
    let func = command["func"].as_str().unwrap();
    let desc = command["desc"].as_str().unwrap();

    exec_func(&func, vec!["hogehoge"]);

    // for command in commands.into_iter() {
    //     let ref func = command["func"];
    //     let ref desc = command["desc"];
    //     println!("{} {}", func, desc);
    // }
}


fn exec_func (func_str: &str, args: Vec<&str>) {
    let mut command = func_str.to_string();
    for i in 0..9 {
        let var_name = format!("${}", i + 1);
        let var_value = args.get(i);
        if let Some(value) = var_value {
            command = command.replace(&var_name, &value).to_string();
        }
    }
    Command::new("sh").arg("-c").arg(command).spawn().expect("failed");
}
