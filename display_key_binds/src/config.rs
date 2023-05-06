use std::fs;

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let paths: Vec<&str> = vec!["keybind.conf", "./res/keyind.conf"];

        let file = match load_from_paths(&paths) {
            Ok(file) => file,
            Err(err) => return Err(err),
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

fn load_from_paths(paths: &Vec<&str>) -> Result<String, String> {
    for path in paths {
        match fs::read_to_string(path) {
            Ok(file) => return Ok(file),
            Err(err) => return Err(err.to_string()),
        };
    }

    Err(String::from("error no config file found"))
}
