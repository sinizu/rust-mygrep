use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Search for query: {}", config.query);
    println!("In file: {}", config.filename);

    // 获取文件中字符串
    let contents: String = fs::read_to_string(config.filename).expect("Failed to read file");
    println!("contents: {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename}
}
