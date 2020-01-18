use std::path::Path;
use std::fs::{File};
use std::io::{ErrorKind, Error, Write, Read};
use std::process;

// 读取文件并展示
pub fn read_file_and_print_2(file_name: String) {
    match File::open(file_name) {
        Ok(mut f)=>{
            let mut content = String::new();
            f.read_to_string(&mut content);
            println!("{:#?}", content);
            let mut stdout = std::io::stdout();
            match stdout.write(&content.as_bytes()) {
                Ok(len) => {
                    println!("--------\nthe string length is {}", len)
                },
                Err(err) => {
                    eprintln!("write content to stdout is error:{}.", err);
                    process::exit(-1);
                }
            }
        },
        Err(err) => {
            eprintln!("open file error:{}.", err);
            process::exit(-1);
        }
    }
}
