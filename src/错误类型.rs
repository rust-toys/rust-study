use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    // let f = File::open("hello.txt").expect("自定义错误信息");
    let f = read_username_from_file("hello.txt".to_string());
    print!("{:?}", f)
}

fn read_username_from_file(path: String) -> Result<String, io::Error> {
    // 首先读取文件
    let mut f = File::open(path)?;
    // 匹配Result
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };
    // 创建需要返回的String
    let mut s = String::new();
    // 读取文件内容
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(err) => Err(err),
    // }
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn match_result_error_kind() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            // 错误类型如果是NotFound，就创建文件
            ErrorKind::NotFound => match File::create("hello.txt") {
                // 匹配创建文件时的Result
                Ok(fc) => fc,
                Err(err) => panic!("文件创建失败{:?}", err),
            },
            other_error => panic!("错误信息为：{:?}", other_error),
        },
    };
    print!("{:?}", f)
}
