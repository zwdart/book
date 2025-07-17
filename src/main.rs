use std::env;
use std::process;

use book::Config;

fn main() {
    // 获取控制台输入参数
    // let args: Vec<String> = env::args().collect();

    // 直接使用返回的迭代器
    // 解析控制台输入参数
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {err}");
        // 标准库提供了 eprintln! 宏来打印到标准错误流，
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    }); 

    if let Err(e) = book::run(config) {
        // println!("Application error: {e}");
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}