use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
pub struct Tilper {
    data: HashMap<String, String>
}

#[allow(dead_code)]
impl Tilper {
    pub fn new() -> Tilper {
        Tilper {
            data: HashMap::new()
        }
    }
    pub fn load_data(&mut self) -> Result<&mut Tilper, String> {
        let data = fs::read_to_string("db").unwrap();
        for line in data.split("\n") {
            if !(line == "") {
                let component: Vec<&str> = line
                    .split("=")
                    .collect();
                let key = component[0].to_string();
                let value = component[1].to_string();
                
                self.data.insert(key, value);
            }
        }

        Ok(self)
    }
    pub fn replace_all(&mut self, map: HashMap<String, String>) -> Result<&mut Tilper, String> {
        let mut results: Vec<String> = vec!();

        for (k, v) in map.iter() {
            results.push(format!("{}={}", k, v));
        }
        
        let data = results.join("\n");
        fs::write("db", data).unwrap();
        Ok(self)
    }
    pub fn set(&mut self, key: String, value: String) -> Result<&mut Tilper, String> {
        match self.data.contains_key(&key) {
            true => Err(format!("There's already a value in the database with the key {}", key)),
            false => {
                self.load_data().unwrap();
                self.data.insert(key, value);
                self.replace_all(self.data.clone()).unwrap();
                Ok(self)
            }
        }
    }
    pub fn get(&self, key: String) -> Result<String, String> {
        match self.data.get(&key) {
            Some(value) => Ok(value.to_string()),
            None => Err(format!("No value in the database with the key {}", key))
        }
    }
}
