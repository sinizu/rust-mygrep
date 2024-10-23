use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Hello, args: {:?}", args);

    // 获取参数
    let query = &args[1];
    let filename = &args[2];

    println!("Search for query: {}", query);
    println!("In file: {}", filename);

    // 获取文件中字符串
    let contents: String = fs::read_to_string(filename)
        .expect("Failed to read file");
    println!("contents: {}", contents);

}


