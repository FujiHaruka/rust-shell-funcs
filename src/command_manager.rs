extern crate serde;
extern crate serde_json;

use json_storage::JsonStorage;
use serde_json::{Value};

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandItem {
    pub index: usize,
    pub func: String,
    pub desc: String,
}

pub struct CommandManager {
    pub commands: Vec<CommandItem>,
    storage: JsonStorage,
}

impl CommandManager {
    pub fn new(storage_path: &str) -> CommandManager {
        let mut storage = JsonStorage::new(storage_path);
        let value = storage.load().unwrap();
        let commands = convert_to_commands(value);
        CommandManager {
            commands: commands,
            storage: storage,
        }
    }

    pub fn get_command(&self, index: usize) -> Option<CommandItem> {
        self.commands.get(index).map(|item| item.clone())
    }

    pub fn update_indexes_by(&mut self, search_word: &str) -> Result<(), String> {
        let includes = |s: &str, pattern: &str| match s.find(pattern) {
            Some(_) => true,
            None => false
        };

        let mut next_index_cursor = 0;
        // not found index is length
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

    pub fn push_command(&mut self, command: CommandItem) -> Result<(), String> {
        self.commands.push(command);
        self.save()
    }

    pub fn delete_command_by_index(&mut self, index: usize) -> Result<(), String> {
        let found = self.find_vec_index(index);
        if let Some(i) = found {
            self.commands.remove(i);
        } else {
            // Ignore
        }
        self.save()
    }

    pub fn show_commands(self) {
        let len = self.commands.len();
        let should_show = |index| index < len;
        for command in self.commands.into_iter() {
            if should_show(command.index) {
                println!("{}    {}", command.index, command.func);
            }
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

    fn save(&mut self) -> Result<(), String> {
        let value = json!(self.commands);
        self.storage.save(&value)
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
