#[derive(Debug)]
pub struct Keybind {
    pub catagori: String,
    name: String,
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

    pub fn set_catagori(&mut self, mut c: String) {
        if c.starts_with('#') {
            let mut c: &str = &c.as_str()[1..c.len()];
            c = c.trim();
            self.catagori = c.to_string();
        } else {
            c = c.trim().to_string();
            self.catagori = c;
        }
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

    pub fn get_keybinds_from_file(file: String) -> Vec<Keybind> {
        let mut keybinds: Vec<Keybind> = Vec::new();
        let mut catagori = String::new();
        let mut change_catagori = false;

        for line in file.split('\n') {
            if let Some(first_char) = line.chars().next() {
                if first_char == '#' {
                    // get catagori
                    if (line.as_bytes()[1] as char) == '#' && !change_catagori {
                        change_catagori = true;
                        continue;
                    }
                    if (line.as_bytes()[1] as char) == ' ' && change_catagori {
                        catagori = line.to_string();
                        continue;
                    }
                    if (line.as_bytes()[1] as char) == '#' && change_catagori {
                        change_catagori = false;
                        continue;
                    }
                    keybinds.push(Keybind::new());
                    // set name
                    if let Some(last) = keybinds.last_mut() {
                        last.set_name(line.to_string());
                        last.set_catagori(catagori.clone());
                    } else {
                        println!("last not there")
                    }
                    continue;
                }
                if !line.starts_with('\t') {
                    if let Some(last) = keybinds.last_mut() {
                        last.add_key(line.to_string());
                    } else {
                        println!("last not there")
                    }
                    continue;
                }
            };
        }
        keybinds
    }
}
