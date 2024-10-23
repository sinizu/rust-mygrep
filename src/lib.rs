use std::error::Error;
use std::fs;

// 动态的返回错误信息，不需要指明错误
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ?能够简化赋值真正的string,还是返回函数错误的err
    let contents = fs::read_to_string(config.filename)?;

    println!("contens: {}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // 泛型后面有一个字符串，说明其是全生命周期，所以退出函数之后还能使用
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // 错误判断
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
