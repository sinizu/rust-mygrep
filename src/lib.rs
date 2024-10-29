use std::error::Error;
use std::fs;

// 动态的返回错误信息，不需要指明错误
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ?能够简化赋值真正的string,还是返回函数错误的err
    let contents = fs::read_to_string(config.filename)?;

    // println!("contens: {}", contents);
    for line in search(&config.query, &contents) {
        println!("line: {}", line);
    }

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::<&str>::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// 测试用例
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_one_result() {
        let query = "language";
        let contents = "\
Rust
is a systems programming language.
else other";

        assert_eq!(
            vec!["is a systems programming language."],
            search(query, contents)
        );
    }

    #[test]
    fn case_no_result() {
        let query = "no_found";
        let contents = "\
Rust
is a systems programming language.
else other";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn case_no_insensetive() {
        let query = "rus";
        let contents = "\
Rust
is a systems programming language.
else other";

        assert_eq!(vec!["Rust"], search_insensetive(query, contents));
    }
}
