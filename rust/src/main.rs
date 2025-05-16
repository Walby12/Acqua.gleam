use serde_json::{Result, Value};
use std::fs;

fn read_json(data: String) {
    let v: Value = serde_json::from_str(&data).unwrap();

    println!("name: {}", v["name"]);
}

fn read_json_file() -> String {
    let data = fs::read_to_string("../main.json").unwrap();
    data
}

fn main() {
    let data = read_json_file();
    read_json(data);
}
