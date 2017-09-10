extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io;
use std::io::{Write, Read};
use serde_json::{Value};

#[derive(Serialize, Deserialize)]
pub struct CommandItem {
    index: usize,
    func: String,
    desc: String
}

pub struct CommandStore {
    commands: Vec<CommandItem>,
    json_path: String,
    json_file: File
}

impl CommandStore {
    pub fn new(json_path: &str) -> CommandStore {
        let mut json_file = File::open(json_path).unwrap();
        let mut commands_json_string = String::new();
        json_file.read_to_string(&mut commands_json_string).unwrap();

        let value: Value = serde_json::from_str(&commands_json_string).unwrap();
        let commands = convert_to_commands(value);

        CommandStore {
            json_path: json_path.to_string(),
            json_file: json_file,
            commands: commands,
        }
    }

    pub fn update_indexes_by(&mut self, search_word: &str) -> io::Result<()> {
        let includes = |s: &str, pattern: &str| match s.find(pattern) {
            Some(_) => true,
            None => false
        };

        let mut next_index_cursor = 0;
        // filtered index is length
        let commands_len = self.commands.len();
        for i in 0..commands_len {
            let found = includes(&self.commands[i].func, search_word);
            if found {
                self.commands[i].index = next_index_cursor;
                next_index_cursor += 1;
            } else {
                self.commands[i].index = commands_len;
            }
        }

        self.save()
    }

    pub fn push_command(&mut self, command: CommandItem) -> io::Result<()> {
        self.commands.push(command);
        self.save()
    }

    pub fn delete_command_by_index(&mut self, index: usize) -> io::Result<()> {
        let found = self.find_vec_index(index);
        if let Some(i) = found {
            self.commands.remove(i);
        } else {
            // Ignore
        }
        self.save()
    }

    pub fn show_commands(self) {
        for command in self.commands.into_iter() {
            println!("{} {}", command.index, command.func);
        }
    }

    fn find_vec_index(&self, command_index: usize) -> Option<usize> {
        for i in 0..self.commands.len() {
            let ref command = self.commands[i];
            if command.index == command_index {
                return Some(i)
            }
        }
        return None
    }

    fn save(&mut self) -> io::Result<()> {
        let json_str = convert_to_json_string(&self.commands);
        let bytes = json_str.as_bytes();
        self.json_file.write_all(bytes)
    }
}

fn convert_to_commands(value: Value) -> Vec<CommandItem> {
    value.as_array()
         .unwrap()
         .into_iter()
         .map(|value_item| CommandItem {
             index: value_item["index"].as_i64().unwrap() as usize,
             func: value_item["func"].as_str().unwrap().to_string(),
             desc: value_item["desc"].as_str().unwrap().to_string(),
         })
         .collect()
}

fn convert_to_json_string(commands: &Vec<CommandItem>) -> String {
    serde_json::to_string_pretty(commands).unwrap()
}
