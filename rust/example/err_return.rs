// Rust的错误返回

// 错误传播运算符?:
//      Rust中?称为错误传播运算符,为了简化Result<T, E>和Option<T>类型的处理。
//
//      ?主要用在返回Result和Option类型的表达式后面(注意,可以直接在Result和Option类型的变量后直接使用?,
//      因为值本身就是一个表达式)
//
//      工作原理：
//          Result类型:
//              (1) 如果Result是Ok(T)类型, ?运算符会从Ok中提取T的值
//              (2) 如果Result是Err(E)类型,?运算符会从函数中提前返回这个Err(E),并把错误传递给调用者
//              示例:
//                  use std::fs::File;
//                  use std::io::{self, Read};
//                  
//                  fn read_username_from_file() -> Result<String, io::Error> {
//                      let mut f = File::open("hello.txt")?;  // 使用?操作符
//                      let mut s = String::new();
//                      f.read_to_string(&mut s)?;
//                      Ok(s)
//                  }

//          Option类型:
//              (1) 如果Option是Some(T)类型, ?运算符从Some中提取T的值
//              (2) 如果Option是None类型, ?运算符会从函数中提前返回None
//              示例:
//              fn process_optional_string(input: Option<String>) -> Option<String> {
//                  let string = input?;            // 如果 input 是 None，这里会直接返回 None
//                  Some(string.to_uppercase())     // 如果 input 是 Some，处理并返回新的 Some
//              }

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("./hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s);
    
    // 上述代码可以在?之后使用链式调用进一步缩短代码
    let mut s = String::new();
    File::open("./hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
