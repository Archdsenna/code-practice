// use std::io;
use std::fs::File;

fn main() {
    // let mut list = String::new();

    // io::stdin().read_line(&mut list).unwrap();     
                       // 在Rust中,unwrap是一个非常常用的方法,它用于处理Option<T>或Result<T>类型的值
                       // 当你调用unwrap方法时,它会检查Result或Option的值
                       //   - 如果是Result::Ok(T)或Option::Some<T>, 它将返回内部的值T
                       //   - 如果是Result::Err(E)或Option::None, 它将导致程序Panic, 并停止执行
                       // 使用unwrap时的注意事项:
                       //   虽然unwrap()在原型开发或测试时非常方便,因为它可以快速的处理错误,但在生产代码中,
                       //   直接使用unwrap通常不是一个好的做法。这是因为它会在遇到错误时导致程序崩溃,而不是
                       //   允许你优雅地处理错误情况。更好的做法是使用match语句或expect方法,后者允许你在触发
                       //   Panic时提供自定义的错误消息,从而使错误更容易理解和调试
    // let result = io::stdin().read_line(&mut list);
    // match result {     // 在生产代码中处理错误情况的方式
    //     Ok(_) => (),
    //     Err(err) => print!("input error: {}", err),
    // }

    // let fd = File::open("./word.txt").unwrap();  // 如果文件不存在,unwrap()会为我们调用panic!
    let fd = File::open("./hello.txt").expect("Error is here"); // expect()也会调用panic!,但是会在默认的panic!信息前
                                                                // 加入我们提供的字符串,有助于确定具体的出错位置
    println!("fd is: {:#?}", fd);
}
