#[derive(Debug)]
pub struct Keybind {
    pub catagori: String,
    pub name: String,
    pub key: Vec<String>,
}

impl Keybind {
    pub fn new() -> Keybind {
        Keybind {
            name: String::new(),
            catagori: String::new(),
            key: Vec::<String>::new(),
        }
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = Self::clean_string(name);
    }

    pub fn set_catagori(&mut self, catagori: &String) {
        self.catagori = Self::clean_string(catagori);
    }

    pub fn add_key(&mut self, key: String) {
        self.key.push(key);
    }

    pub fn print(&self) {
        println!("Name: {}", self.name);
        println!("catagori: {}", self.catagori);
        for key in &self.key {
            println!("Key: {key}");
        }
    }
    fn clean_string(string: &String) -> String {
        if string.starts_with('#') {
            string.as_str()[1..string.len()].trim().to_lowercase()
        } else {
            string.trim().to_string()
        }
    }
}
