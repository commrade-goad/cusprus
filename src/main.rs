use core::panic;
use rofi;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Write;
use std::process;

#[derive(Deserialize, Serialize)]
struct Config {
    prompt_name: String,
    pretty_name: Vec<String>,
    command: Vec<String>,
}

impl Config {
    fn default_value() -> Config {
        return Config {
            prompt_name: "Special Menu".to_string(),
            pretty_name: vec!["Get Username".to_string(), "Get Kernel".to_string()],
            command: vec!["notify-send $USER".to_string(), "notify-send \"$(uname -a)\"".to_string()],
        };
    }
}

fn read_args() -> Vec<String> {
    return std::env::args().collect();
}

fn read_configuration(path_to_config: &String) -> Option<Config> {
    let mut config = String::new();
    let path_to_def_conf = match env::var("HOME") {
        Ok(val) => format!("{val}/.config/cusprus.toml"),
        Err(err) => panic!("Error : Failed to read $HOME env var => {err}"),
    };
    match &path_to_config[..] {
        "None" => {
            config.push_str(&path_to_def_conf);
        }
        _ => {
            config.push_str(path_to_config);
        }
    }
    if std::path::Path::new(&config).is_file() == false {
        if std::path::Path::new(&path_to_def_conf).is_file() == true {
            println!(
                "{}",
                format!("Error : Configuration file at `{}` doesnt exist!", &config)
            );
            process::exit(1);
        } else {
            println!("Error : Configuration file at `{}` doesnt exist!", &config);
            let mut create_config = fs::File::create(&path_to_def_conf)
                .expect("Error : Failed to create the configuration file!");
            let default_conf = toml::to_string(&Config::default_value()).map_err(|e| {
                eprintln!("ERROR: Failed to Deserialize the default config!");
                return e;
            });
            if default_conf.is_ok() {
                let _ = create_config.write_all(default_conf.unwrap().as_bytes());
            }
            println!("Info : Example configuration file created at `{path_to_def_conf}`.");
            return None;
        }
    }
    let read_file = match fs::read_to_string(config) {
        Ok(val) => val,
        Err(err) => panic!("Error : Failed to read the configuration file => {err}"),
    };
    let parse_file: Config = match toml::from_str(&read_file) {
        Ok(val) => val,
        Err(err) => panic!("Error : Failed to parse the file => {err}"),
    };
    return Some(parse_file);
}

fn spawn_rofi(pretty_name: Vec<String>, prompt_name: String) -> usize {
    let selected = rofi::Rofi::new(&pretty_name)
        .prompt(prompt_name)
        .run_index();
    let return_value = match &selected {
        Ok(val) => val,
        Err(err) => panic!("Error : Rofi error => {err}"),
    };
    return return_value.to_owned();
}

fn main() {
    let mut user_args: Vec<String> = read_args();
    if user_args.len() < 2 {
        user_args.push("None".to_string());
    }
    let configuration = read_configuration(&user_args[1]);
    if !configuration.is_some() {
        std::process::exit(1);
    }
    let conf_unwraped = configuration.unwrap();
    let value = spawn_rofi(conf_unwraped.pretty_name, conf_unwraped.prompt_name);
    std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("{}", conf_unwraped.command[value]))
        .spawn()
        .expect("Error : Failed to spawn the process");
}
