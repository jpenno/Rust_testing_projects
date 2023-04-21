#[cfg(test)]
mod tests {
    use crate::config::*;
    use crate::keybind::*;
    use crate::keybinds::*;

    #[test]
    fn test_config_path() {
        let config: Config = match Config::new() {
            Ok(config) => config,
            Err(err) => {
                println!("Config: {err}");
                return;
            }
        };
        assert_eq!(config.path, String::from("/home/jp/.config/sxhkd/sxhkdrc"));
    }

    #[test]
    fn test_set_catagori() {
        let mut keybind = Keybind::new();

        keybind.set_catagori(&String::from("# Media"));
        assert_eq!(keybind.catagori, "media");

        keybind.set_catagori(&String::from("media"));
        assert_eq!(keybind.catagori, "media");

        keybind.set_catagori(&String::from(""));
        assert_eq!(keybind.catagori, "");
    }

    #[test]
    fn test_set_name() {
        let mut keybind = Keybind::new();

        keybind.set_name(&String::from("# emacs"));
        assert_eq!(keybind.name, "emacs");

        keybind.set_name(&String::from("emacs"));
        assert_eq!(keybind.name, "emacs");

        keybind.set_name(&String::from(""));
        assert_eq!(keybind.name, "");
    }

    #[test]
    fn test_get_keybinds_from_file() {
        let keybinds = Keybinds::new(&String::from("././test_data/keybind.conf"));

        assert!(keybinds.is_ok());
        if let Ok(k) = keybinds {
            assert_eq!(k.keybinds[0].name, "runelite");
            assert_eq!(k.keybinds[0].catagori, "game keys");
            assert_eq!(k.keybinds[0].name, "runelite");
            assert_eq!(k.keybinds[0].key[0], "super + g ; r");

            assert_eq!(k.keybinds[2].name, "kitty");
            assert_eq!(k.keybinds[2].catagori, "programms");
            assert_eq!(k.keybinds[2].key[0], "super + Return");
            assert_eq!(k.keybinds[2].key[1], "super + t");
        }
    }
}
