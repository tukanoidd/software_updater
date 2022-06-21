use std::{fs::File, io::Write};

use software_updater_config::Config;

fn main() {
    let config_path = dirs::config_dir()
        .unwrap()
        .join("software_updater")
        .join("config.json");

    if !config_path.exists() {
        std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();

        let mut config_file = File::create(config_path).unwrap();
        let config = Config::default();
        let config_toml = serde_json::to_string_pretty(&config).unwrap();

        config_file.write_all(config_toml.as_bytes()).unwrap();
    }
}
