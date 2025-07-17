// 注释包含项的结构
// 文档注释风格 //! 为包含注释的项，而不是位于注释之后的项增加文档。这通常用于 crate 根文件（通常是 src/lib.rs）或模块的根文件为 crate 或模块整体提供文档。

//! # Rust Book
//!
//! `book` is a collection of utilities to make performing certain
//! calculations more convenient.

use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// 处理控制台输入参数
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // 第一个是程序路径
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // 带环境变量参数
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

// 处理数据逻辑
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{}", contents);

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // 在 run 函数中使用 search 函数
    for line in results{
        println!("{line}");
    }

    Ok(())
}

// 搜索关键词
// 使用迭代器适配器让代码更清晰
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 这段代码使用 filter 适配器来保留 line.contains(query) 返回 true 的行。接着使用 collect 将匹配行收集到另一个 vector 中。
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 搜索关键词，大小写不敏感
pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str,
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

// 编写测试
#[cfg(test)]
mod tests {
    use super::*;

    // 搜索测试
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
      assert_eq!(vec!["safe, fast, productive."], search(query, contents));  
    }

    // 大小写不敏感测试
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."],
         search_case_insensitive(query, contents)
        );
    }
}



// src/utils.rs
pub fn helper() {  // 注意 `pub` 使函数对外可见
    println!("Utils function called");
}

// 文档注释作为测试
// 在文档注释中增加示例代码块是一个清楚的表明如何使用库的方法，这么做还有一个额外的好处：cargo test 也会像测试那样运行文档中的示例代码！

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = minigrep::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}