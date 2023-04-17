use std::fs;

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let path = String::from("./res/keybind.conf");

        let file: String = match fs::read_to_string(&path) {
            Ok(in_file) => in_file,
            Err(err) => {
                println!("Path: {:?}", path);
                print!("Err: {err}");
                return Err(format!("Path: {}, Err: {}", path, err));
            }
        };

        let mut config = Config {
            path: String::new(),
        };
        for line in file.split('\n') {
            if line.contains("path") {
                let mut path = String::new();
                let mut copy = false;
                for c in line.chars() {
                    if c == '"' {
                        copy = true;
                    }
                    if copy && c != '"' {
                        path.push(c);
                    }
                }
                config.path = path;
            }
        }
        Ok(config)
    }
}
