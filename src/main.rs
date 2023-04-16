use core::panic;
use rofi;
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::io::Write;
use std::process;

#[derive(Deserialize)]
struct Config {
    prompt_name: String,
    pretty_name: Vec<String>,
    command: Vec<String>,
}

fn read_args() -> Vec<String> {
    return std::env::args().collect();
}

fn read_configuration(path_to_config: &String) -> (Vec<String>, Vec<String>, String) {
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
            println!("{}", format!("Error : Configuration file at `{}` doesnt exist!",&config));
            process::exit(1);
        } else {
            println!("Error : Configuration file at `{}` doesnt exist!",&config);
            let mut create_config = fs::File::create(&path_to_def_conf)
                .expect("Error : Failed to create the configuration file!");
            create_config.write_all(b"prompt_name = \"Special Menu\"\npretty_name = [\"Example Name\", \"Example 2\"]\ncommand = [\"echo hello $USER\", \"echo this is example 2\"]").expect("Error : Failed to write the configuration file");
            println!("Info : Example configuration file created at `{path_to_def_conf}`.");
            process::exit(1);
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
    return (parse_file.pretty_name, parse_file.command, parse_file.prompt_name);
}

fn spawn_rofi(pretty_name: Vec<String>, prompt_name:String) -> usize {
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
    let value = spawn_rofi(configuration.0, configuration.2);
    std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("{}", configuration.1[value]))
        .spawn()
        .expect("Error : Failed to spawn the process");
}
