use std::collections::HashMap;

mod pulling;

#[derive(Debug)]
pub struct Registry {
    raw_data: HashMap<String, f32>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry { raw_data: HashMap::new() }
    }

    pub fn insert(&mut self, (key, value): (String, f32)) {
        self.raw_data.insert(key, value);
    }
}
