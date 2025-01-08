// std::io模块包含了许多在执行输入和输出时需要的常见操作,
// 该模块最核心的部分是Read和Write traits。
//
// 因为它们是traits, 所以Read和Write由许多其他类型实现, 我们可以看到几种不同类型的I/O：
//  File、TcpStream、Vec<T>
//
// Read添加了read方法，我们可以在File上使用该方法, 如下:

use std::io;
use std::fs::File;

fn main() -> io::Result<()> {

}
