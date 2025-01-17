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

use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("./hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s);
    
    // 上述代码可以在?之后使用链式调用进一步缩短代码
    let mut s = String::new();
    File::open("./hello.txt")?.read_to_string(&mut s)?;  // std::io::Read 的 trait
    // Ok(s)将创建一个包含文件内容的Result::Ok，并返回这个值
    Ok(s)    // Ok(s)用于将函数的成功结果封装成一个Result类型
             // 在Rust中,Ok(s)是Result类型的一个构造器,用于表示操作成功完成,并且包含成功时的值

    // 上面的代码可以进一步简化
    fs::read_to_string("hello.txt");   // std::fs::read_to_string()

    // 上面代码中有两个read_to_string的调用,但是它们是不同的两个函数,它们属于不同的Rust标准库部分,并且实现方式不同
    // 第一个read_to_string()是std::io::Read trait的一个方法,
    // 第二个read_to_string()是std::fs模块的一个函数,这是一个高级的辅助函数,用于直接读取整个文件的内容到一个新的字符串
    // std::io::Read trait 的 read_to_string()需要一个已经打开的Read类型的实例
    // std::fs::read_to_string()是一个独立的函数,处理文件的打开、读取和关闭。它会打开文件,新建一个String,读取
    //                          文件的内容,并将内容放入String,接着返回它
}

// main函数的返回类型是有限制的。main函数的一个有效返回值是(),同时出于方便,另一个有效的返回值是Result<T, E>
fn main() -> Result<(), Box<dyn Error> {   // Box<dyn Error>为使用?时,main允许返回的“任何类型的错误”
    let f = File::open("./hello.txt")?;  // 如果成功,?会从Ok(T)中提取T的值,如果失败,则会直接返回Err(e)
    Ok(())
}
