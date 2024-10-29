use mygrep::Config;
use std::env;
use std::process;

// main函数中只需要保留
// 1.接受参数
// 2.调用run
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // 闭包，如果返回的是Err(e),那么内部字符串e会传送给err
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    println!("Search for query: {}", config.query);
    println!("In file: {}", config.filename);

    // 获取文件中字符串
    if let Err(e) = mygrep::run(config) {
        eprintln!("fn run failed: {}", e);

        process::exit(1);
    }
}
