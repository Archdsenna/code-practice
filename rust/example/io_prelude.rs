// Q: use std::io::prelude; 与use std::io::prelude::*; 这两种写法的区别是什么?
// A: (1) use std::io::prelude;
//        这种写法仅仅是将prelude模块本身作为一个模块导入到当前的命名空间,而不导入模块内的任何项,
//        这意味着如果要使用这些trait的方法,必须要引用完整的路径
//
//        这种写法通常用于导入一组特定的trait, 这些trait对于输入/输出操作非常有用
//        prelude模块是Rust标准库中的一个特殊模块, 它包含了一些最常用的trait和类型,使得你可以更方便的使用这些功能
//
//        std::io::prelude的具体内容:
//          std::io::prelude模块包含了几个常用的I/O相关的trait, 例如:
//              - Read    : 提供读取数据的方法
//              - Write   : 提供写入数据的方法
//              - BufRead : 提供缓冲读入的方法,允许更高效地读取大块数据
//              - Seek    : 提供移动I/O光标位置的方法
//   (2) use std::io::prelude::*;
//       这种写法的目的是将prelude模块中所有的公共项(通常是trait)导入到当前作用域中,
//       这意味着你可以直接在你的代码中使用这些trait的方法, 如read(),write()等, 而无需再次明确指定它们的路径
// 
// 总结:
//      使用 use std::io::prelude::*; 可以将 prelude 模块中的所有公共项导入当前作用域，便于直接使用这些功能。
//      使用 use std::io::prelude;    只是将 prelude 模块作为一个命名空间导入，需要使用完整路径来访问模块中的项。

use std::io;
use std::io::prelude;           // 仅导入prelude模块,没有导入任何项
// use std::io::prelude::*;        // 导入prelude模块下的所有公共项
use std::fs::File;

fn main() -> io::Result<()> {
// fn main() {
    let mut file = File::open("mod_io_text.txt")?;
    let mut buffer = [0; 10];
    // file.read(&mut buffer)?;                        // 导入prelude::*
    prelude::Read::read(&mut file, &mut buffer)?;    // 只导入prelude
    println!("buffer: {:?}", buffer);
    Ok(())
}
