// 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
// 所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
//
// 元音字母: a e i o u, 其余为辅音字母

use std::io;

/// V1版本
// fn main() {
//     let vowel = "aeiou";
//     let mut str = String::new();
//
//     // io::stdin().read_line(&mut list).unwrap();     
//                        // 在Rust中,unwrap是一个非常常用的方法,它用于处理Option<T>或Result<T>类型的值
//                        // 当你调用unwrap方法时,它会检查Result或Option的值
//                        //   - 如果是Result::Ok(T)或Option::Some<T>, 它将返回内部的值T
//                        //   - 如果是Result::Err(E)或Option::None, 它将导致程序Panic, 并停止执行
//                        // 使用unwrap时的注意事项:
//                        //   虽然unwrap()在原型开发或测试时非常方便,因为它可以快速的处理错误,但在生产代码中,
//                        //   直接使用unwrap通常不是一个好的做法。这是因为它会在遇到错误时导致程序崩溃,而不是
//                        //   允许你优雅地处理错误情况。更好的做法是使用match语句或expect方法,后者允许你在触发
//                        //   Panic时提供自定义的错误消息,从而使错误更容易理解和调试
//     let ret= io::stdin().read_line(&mut str);
//     match ret{
//         Ok(_) => (),
//         Err(err) => print!("input error: {}", err),
//     }
//
//     let words = str.split_whitespace();
//     let mut pig_str = String::new();
//
//     for word in words {
//         let mut word_str = word.to_string();
//         let mut chars = word.chars();
//         let first_char = chars.next().unwrap();
//         if vowel.contains(first_char) {   // 处理元音字母
//             word_str.push_str("-hay ");
//             pig_str.push_str(&word_str);
//             // println!("pig-word: {}", pig_str);
//         } else {                          // 处理辅音字母
//             // println!("no {}", first_char);
//             let sub_word = chars.as_str();
//             let mut sub_word_str = sub_word.to_string();
//             sub_word_str.push('-');
//             sub_word_str.push(first_char);
//             sub_word_str.push_str("ay ");
//             // println!("sub word: {}", sub_word_str);
//             pig_str.push_str(&sub_word_str);
//         }
//     }
//
//     pig_str.pop();  // 移除最后一个多余的空格
//     println!("pig string is : {}", pig_str);
// }


/// V2版本: 优化版本
fn main() {
    let vowels = String::from("aeiou");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    
    let mut pig_str = String::new();

    // 去除首尾的空格并且按空格分隔单词
    let words = input.trim().split_whitespace();

    for word in words {
        let first_char = word.chars().next().unwrap();
        if vowels.contains(first_char) {
            pig_str.push_str(&word.to_string());
            pig_str.push_str("-hay ");
        } else {
            let mut chars = word.chars();
            chars.next();   // 过掉第一个辅音字符
            pig_str.push_str(&chars.as_str());
            pig_str.push('-');
            pig_str.push(first_char);
            pig_str.push_str("ay ");
        }
    }

    if !pig_str.is_empty() {
        pig_str.pop();          // 移除多余的空格
    }

    println!("Pig Latin string is {}", pig_str);
}
