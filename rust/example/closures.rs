// Rust中的闭包(closures)
// Rust的闭包是一种可以捕获其环境的匿名函数
//
// 如何使用闭包：
//  闭包通常用||符号定义，闭包体放在{}中。
//  闭包的多个参数在||中使用,分隔,例如|x, y, z|
//  闭包的参数默认是不可变的
//  闭包可以直接调用,也可以作为参数传递给其他函数
//  闭包是通过闭包表达式定义的,但它们的行为类似于函数,它们可以被调用并传递参数
// 
// 闭包的返回值:
//  在Rust中,
//  闭包的返回值是最后一个表达式的值,除非最后一个表达式以分号;结束,这种情况下闭包的返回值是()(即单元类型)

fn main() {
    // let add = |x, y| x + y;      // |x, y| x +y是一个闭包表达式, 而add称为一个闭包或闭包实例
                                    // 当包体只有一个语句时,可以不使用{}来包含包体
    // let add = |x, y| {x + y};    // 与上面效果相同
    let add = |mut x, y| {    // 这个闭包有名字,但通常闭包是匿名的,例如:
                              // let list_len = list.chars().filter(|c| !c.is_whitespace()).count();只有闭包表达式,没有闭包名字
        x += y;
        x
    };

    let sum = add(9, 9);
    println!("sum is {}", sum);

    // 将闭包作为参数传递给另一个函数
    let numbers = vec![1, 2, 3];
    let sum = apply_to_vec(numbers, |x| x + 1);
    println!("vector: {:?}", sum);
}

// <F>是泛型类型参数的声明方式
fn apply_to_vec<F>(vec: Vec<i32>, mut func: F) -> Vec<i32>  // mut func的含义: Rust中变量默认是不可变的,
                                                            // 如果需要修改一个变量,必须在声明时用mut关键字来标记它为可变的
                                                            // 这同样适用于函数参数,表明函数可能会改变其内部状态
where       // where子句的主要作用是提供一种声明泛型参数约束的方式
    F: FnMut(i32) -> i32,
{
    let mut result = Vec::new();   // 一般情况下,定义一个Vec<T>类型的变量,需要显式的声明其元素类型,
                                   // 如果没有显式声明元素类型, 后续必须有使用它进行类型特定的操作，这样Rust编译器会推断出Vec<T>的类型
    for value in vec.iter() {
        result.push(func(*value));
    }
    result
}
