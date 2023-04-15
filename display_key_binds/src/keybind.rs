#[derive(Debug)]
pub struct Keybind {
    name: String,
    catagori: String,
    key: Vec<String>,
}

impl Keybind {
    pub fn new() -> Keybind {
        Keybind {
            name: String::new(),
            catagori: String::new(),
            key: Vec::<String>::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_catagori(&mut self, c: String) {
        self.catagori = c;
    }

    pub fn add_key(&mut self, key: String) {
        self.key.push(key);
    }

    pub fn print(&self) {
        println!("Name: {}", self.name);
        println!("catagori: {}", self.catagori);
        for key in &self.key {
            println!("Key: {}", key);
        }
    }
}
