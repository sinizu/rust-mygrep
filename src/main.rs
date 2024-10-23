use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Hello, args: {:?}", args);

    // 获取参数
    let query = &args[1];
    let filename = &args[2];

    println!("Search for query: {}", query);
    println!("In file: {}", filename);
}


