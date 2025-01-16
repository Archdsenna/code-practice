// panic时打印backtrace:
//  $ RUST_BACKTRACE=1 cargo run
//
//  补充: cargo build (或cargo run)的--release选项的作用:
//      Rust项目发布构建时,会有两种模式,默认的模式为debug模式(不带--release),以及release模式,即发布模式(带--release),
//      发布模式编译出的二进制性能更好,但是编译过程可能会比debug模式耗时。

use std::{fs::File, io::ErrorKind};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let fd = match File::open("./hello.txt") {
        // Result<T, E>类型和Option<T>类型一样(它们都是枚举类型),都被导入到了prelude模块中,
        // 因此不需要在match分支的Ok和Err之前指定Result::了
        Ok(fd) => fd,
        // 针对错误的不同情况分别进行处理
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("./hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // other_error在这里用作一个通用捕获变量,它匹配ErrorKind枚举中的所有其他可能的错误
            other_error => panic!("Problem Opening the file: {:?}", other_error),
        }
    };

    // 用闭包的方式优化上面使用match处理错误的逻辑
    // unwrap_or_else()是一个错误处理方法,它允许你定义一个闭包,该闭包将在Result是Err时执行
    // unwrap_or_else()函数的工作原理:
    //  如果Result是Ok类型,则返回包含在Ok中的值,
    //  如果是Err类型,它会执行你提供的闭包函数
    let f = File::open("./word.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("./word.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
