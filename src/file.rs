use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;
use std::process;

// 读取文件并展示
pub fn read_file_and_print_2(file_name: String) -> std::io::Result<usize> {
    match File::open(file_name) {
        Ok(mut f) => {
            let mut content = String::new();
            f.read_to_string(&mut content)?;
            let mut stdout = std::io::stdout();
            match stdout.write(&content.as_bytes()) {
                Ok(len) => println!("--------\nthe string length is {}", len),
                Err(err) => {
                    eprintln!("write content to stdout is error:{}.", err);
                    return Ok(1);
                }
            }
        }
        _ => {
            eprintln!("open file error.");
            return Ok(2);
        }
    }
    Ok(0)
}
