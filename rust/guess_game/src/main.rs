// Tips: 
//  1. Q: 如何查看依赖中的文档?
//     A: 使用cargo doc --open 命令,会在浏览器中打开该项目所有依赖的说明文档,点击左侧即可浏览
use std::io;
use std::cmp::Ordering;   // (1) 并非所有类型都实现了 Ord trait。例如，浮点数类型 f32 和 f64 实现了 PartialOrd 而不是 Ord，
                          //     因为浮点数的比较可能不是完全有序的（例如 NaN 的比较问题）。
                          // (2) 自定义类型默认不实现 Ord。如果你需要在自定义类型上使用 cmp 方法，
                          //     你需要手动实现 Ord 和 PartialOrd trait。
// 可以使用嵌套路径来消除大量的use行
// use std::{io, cmd::Ordering};    // 可以使用嵌套路径将相同的项在一行中引入作用域,这么做需要指定路径的相同部分,
                                    // 接着是两个冒号,接着是大括号中的各自不同的路径部分

// 我们可以在路径的任何层级使用嵌套路径, 例如可以将具有相同路径的引用使用嵌套路径合为一句:
//      use std::io;
//      use std::io::Write;
//
//      use std::{self, Write};  // 使用嵌套路径合为一句, 路径相同的部分使用self
//

use rand::Rng; // 为什么要引入Rng？
               // 在 Rust 中，如果你想使用某个类型的方法，那么这个类型对应的 trait 必须在作用域中。即使 thread_rng() 返回的类型实现了 Rng trait，如果没有将 Rng trait 引入作用域，编译器不会允许你调用 gen_range 等 Rng trait 定义的方法。
               // 通过 use rand::Rng; 引入 Rng trait 后，你就可以在当前作用域内使用所有由 Rng 提供的方法

struct Guess {
    value: i32,
}

impl Guess {   // 定义结构体的方法
    fn new(value: i32) -> Guess {   // 关联函数,new()通常称为构造函数
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { 
            value    // 结构体字段初始化简写方法,如果字段名和变量名相同,则可以省略字段名和冒号,
                     // 直接使用变量名作为字段值
        }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {    // 使用fn定义一个函数
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is {}", secret_num);

    println!("Please input you guess.");

    loop { // loop用来创建一个无限循环,循环体用{}括号括起来
        // 使用mut修饰变量,表示变量可变,rust变量默认不可变
        let mut guess = String::new();  // 调用String类型的new方法,创建一个新的空字符串

        io::stdin()                     // io::stdin方法返回一个io::Stdin实例
            // &用于创建一个值的引用,而不是传递值本身,这里使用引用的原因主要涉及到 Rust 的所有权和借用规则
            // 为什么使用引用：
            //  (1) 避免所有权转移：
            //      在 Rust 中，变量默认拥有其数据的所有权。当一个变量被传递到函数或方法中时，除非特别指定，否则它的所有权会被转移。使用引用可以避免所有权的转移，允许原始变量在方法调用后继续使用。
            //  (2) 减少内存消耗和提高性能：
            //      直接传递大型数据结构（如大数组、字符串、复杂结构体等）可能会涉及到数据的复制，这会增加内存使用并可能影响性能。
            //      通过传递引用，无论原始数据结构的大小如何，都只传递一个固定大小的引用，从而提高效率。
            //
            //  Rust的可变引用与不可变引用
            //      (1) 不可变引用: &
            //      (2) 可变引用:   &mut
            .read_line(&mut guess)      // 调用io::Stdin实例的read_line方法,该方法返回一个Result类型,此处为io::Result
            .expect("Failed to read."); // Result类型有expect方法,如出错可使程序Panic。Result类型是一个枚举型的变量,包含Ok和
                                        // Err两个类型,如果正确会返回Ok中的值,如果出错会打印错误消息

        // rust变量注解: 在变量名后使用:分隔,然后跟上类型名称,以明确变量的数据类型
        // rust变量遮蔽(Variable Shadowing): 结合 let mut guess = String::new()
        //  使用let关键字再次声明同名变量是一种变量遮蔽的特性,允许在相同的作用域内重新声明一个新的变量,
        //  使用相同的名称但可以拥有不同的类型或值, 广泛应用于类型转换或值的重新赋值
        let guess: i32 = match guess.trim().parse() { // Rust编译器会根据这个类型注解尝试将字符串转换为u32类型
            Ok(num) => {
                let guess = Guess::new(num).value();
                guess
            },   
                                // match表达式每个分支的模式可以引入新的变量,这些变量是在模式匹配成功时创建和初始化的,
                              // 这些变量的作用域仅限于相应的 match 分支
                              // 这是 Rust 语言模式匹配特性的一部分，允许直接从匹配的数据结构中提取值。
            Err(_) => continue, // _ 是一个通配符，表示我们不关心具体的错误信息
        };   // 注意这里的语句块里有分号结尾,因为这里是一个语句结束
        println!("You guessed: {}", guess);

        // 字符串实例的另一种写法
        // let guess: u32 = "666".parse().expect("This is error message!");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),  // 使用match语句时,每个分支以','号结束
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {        // 如果需要执行多条语句,用{}括号把多条语句括起来
                println!("You win!");
                break;
            }
        }
    }
}
