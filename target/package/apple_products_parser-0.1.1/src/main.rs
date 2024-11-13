use serde::{Deserialize, Serialize};
use serde_yaml::to_string as to_yaml;
use std::{env, fs::File, io::Read};
use toml::to_string as to_toml;

#[derive(Debug, Deserialize, Serialize)]
struct Product {
    name: String,
    price: f64,
    #[serde(rename = "type")]
    product_type: String,
    screen_size: String,
    storage: Vec<String>,
    color: Vec<String>,
    date_of_release: String,
    availability: bool,
}

fn parse_file_to_yaml(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();

    let products: Vec<Product> = serde_json::from_str(&json).unwrap();
    let yaml = to_yaml(&products).unwrap();
    println!("YAML: \n{}", yaml);
}

fn parse_file_to_toml(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();

    let products: Vec<Product> = serde_json::from_str(&json).unwrap();
    let toml = to_toml(&products).unwrap();
    println!("TOML: \n{}", toml);
}

fn info() {
    println!("Apple Products Parser:");
    println!("\nCommands:");
    println!(" cargo run parse devices.json yaml   Parse a file and convert to yaml");
    println!(" cargo run parse devices.json toml   Parse a file and convert to toml");
    println!(" cargo run help                      Display help information");
    println!(" cargo run credits                   Display credits");
    println!(" cargo test                          Print tests");
}

fn show_credits() {
    println!("This parser is done by Daria Filozop");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        info();
        return;
    }
    let command = &args[1];
    match command.as_str() {
        "parse" => {
            if args.len() < 4 {
                eprintln!("You have forgotten about file or format argument for command.");
                info();
            }

            let filename = &args[2];
            let format = &args[3];

            match format.as_str() {
                "yaml" => parse_file_to_yaml(filename),
                "toml" => parse_file_to_toml(filename),
                _ => {
                    eprintln!(
                        "Please, choose another format (only yaml or toml): {}",
                        format
                    );
                    info();
                }
            }
        }
        "help" => {
            info();
        }
        "credits" => {
            show_credits();
        }
        _ => {
            eprintln!("Unknown command '{}'", command);
            info();
        }
    }
}
