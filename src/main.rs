use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // 闭包，如果返回的是Err(e),那么内部字符串e会传送给err
        println!("Error: {}", err);
        process::exit(1);
    });

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
    // 泛型后面有一个字符串，说明其是全生命周期，所以退出函数之后还能使用
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // 错误判断
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
