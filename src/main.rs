use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

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

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
