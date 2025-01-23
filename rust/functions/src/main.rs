// Rust中函数和方法是不同的概念:
//  函数：独立的函数体，不属于任何类型,即不与任何类型关联
//  方法: 属于某个类型, 在impl中实现
//  关联函数: 属于某个类型, 在impl中实现
//
//  impl Point {
//     // 关联函数，通常用作构造器, Result<T, E>类型的Ok和Err就是构造器
//     fn new(x: i32, y: i32) -> Point {
//         Point { x, y }
//     }
//
//     // 实例方法，访问 self 来获取实例的数据
//     fn distance_from_origin(&self) -> f64 {
//         ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
//     }
// }
//
// functions
fn five() -> i32 {
    // 6就是一个表达式,简单理解为：表达式是值或者可以计算出值,而语句不会返回值
    6       // 切记,表达式之后没有分号,语句之后才有分号
            // 大部分函数会隐式返回最后一个表达式
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1;   如果改为语句则会报错,报错为类型不匹配,因为函数要返回一个i32类型,但是如果没有返回任何
    //          其他值,则会返回单元值()。单元值()是元组的特殊类型，不包含任何值。
    //          如果改为x + 1;,则表示语句,语句不返回任何值,此值由单元类型()表示,表示不返回值
}

fn print_val(x: i32, c: char) {
    println!("x: {} c: {}", x, c);
}

fn main() {
    print_val(5, 'A');

    let y = five();
    println!("y is {}", y);

    let z = plus_one(8);
    println!("z is {}", z);
}
