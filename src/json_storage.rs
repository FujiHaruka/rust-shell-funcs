extern crate serde;
extern crate serde_json;

use std::fs::{OpenOptions};
use std::io::{Write, Read};
use serde_json::{Value};

pub struct JsonStorage {
    path: String,
}

impl JsonStorage {
    pub fn new(path: &str) -> JsonStorage {
        JsonStorage {
            path: path.to_string(),
        }
    }

    pub fn load(&mut self) -> Result<Value, String> {
        let mut json_string = String::new();
        OpenOptions::new()
            .read(true)
            .open(&self.path)
            .map_err(|e| e.to_string())
            .and_then(|mut file| {
                file.read_to_string(&mut json_string)
                    .map_err(|e| e.to_string())
            })?;

        serde_json::from_str(&json_string)
            .map_err(|err| err.to_string())
    }

    pub fn save(&mut self, value: &Value) -> Result<(), String> {
        serde_json::to_string_pretty(&value)
                   .map_err(|err| err.to_string())
                   .and_then(|json_str| {
                       let bytes = json_str.as_bytes();
                       OpenOptions::new()
                           .write(true)
                           .open(&self.path)
                           .map_err(|e| e.to_string())
                           .and_then(|mut file| {
                               file.write_all(bytes)
                                   .map_err(|e| e.to_string())
                           })
                   })
    }
}

#[cfg(test)]
mod tests {
    use json_storage::JsonStorage;
    use serde_json::Value;

    #[test]
    fn it_works() {
        let mut storage = JsonStorage::new("tmp/test_data.json");
        let data = json!(["abc", "edf"]);
        storage.save(&data).unwrap();
        let loaded = storage.load().unwrap();
        assert_eq!(loaded, data);
    }
}
