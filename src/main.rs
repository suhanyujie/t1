extern crate clap;
extern crate rust_php;

mod file;

use crate::file::read_file_and_print_2;
use clap::{App, Arg};
use rust_php::string::file::file_get_contents;
use std::collections::HashMap;
use std::env;
use std::io::{Error, ErrorKind};
use std::process;

fn main() {
    //    let match_result = file_get_contents(&"Cargo.toml");
    //    match match_result {
    //        Ok(is_match) => {
    //            println!("{}", is_match);
    //        },
    //        Err(err) => {
    //            eprintln!("{}", err)
    //        }
    //    }
    simple_tool();
    //parse_novel();
}

/// 简单的命令行工具
fn simple_tool() {
    let matches = App::new("t1")
        .version("0.1.0")
        .author("suhanyu")
        .about("简单的命令行工具，用于查找一段时间内的日志")
        .arg(
            Arg::with_name("FILE")
                .help("FILE to print.")
                .empty_values(false),
        )
        .get_matches();

    let file_name = matches.value_of("FILE").unwrap_or("");
    read_file_and_print_2(String::from(file_name));
}

/// 解析 novel
fn parse_novel() {
    let url = "";
    let html = String::from("");
    let data = parse_html(&html);
    println!("{:#?}", data);
}

/// 获取网页内容，并解析成结构化数据
fn parse_html(html: &str) -> Result<HashMap<String, String>, Error> {
    let mut one_res = HashMap::new();
    if let a = one_res.insert("key".to_string(), "value".to_string()) {
        return Ok(one_res);
    }
    Err(Error::new(ErrorKind::Other, "something error"))
}

/// 数组相关的测试代码
fn vec_test() {
    // 命令行参数
    let env_args = env::args();
    println!("{:#?}", env_args);
    // 使用宏定义数组
    let arr1 = vec![1, 2, 3];
    println!("{:#?}", arr1);
    // 声明数组时，可以指定类型，并且初始化每一个值
    let arr1: [i32; 10] = [0; 10]; //
    println!("{:#?}", arr1);
    println!("Hello, world!");
}

// 字符串的使用测试
fn str_test() {
    let s1 = "this is 露可娜娜".to_string();
    println!("{:#?}", s1.chars().nth(8).unwrap());
}
