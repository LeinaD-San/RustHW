use std::fs::File;
use std::io::prelude::*;

struct Config {
    name: String,
    id: String,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
       // let api_key = lines.next().unwrap().to_string();
        //let port = lines.next().unwrap().parse().unwrap();
        let id = lines.next().unwrap().to_string();
        Config { name, id }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("username: {}", config.name);
    //println!("api key: {}", config.api_key);
    //println!("port: {}", config.port);
    println!("id: {}", config.id);
}
fn main() {
    reading_from_file();
}
