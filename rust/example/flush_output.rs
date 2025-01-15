// Rust中使用print!可能存在的问题:
// Rust代码中当使用print!宏打印时，如果后面紧跟着有一个输入语句，可能会出现输入先于print!的内容出现

use std::io;

fn main() {
    let mut word = String::new();
    // print!("Please input a word: \n");
    println!("Please input a word: ");

    io::stdin().read_line(&mut word).unwrap();
    println!("word is {}", word);
}
