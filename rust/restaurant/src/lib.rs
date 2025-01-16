// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// 模块结构
// crate    // 整个模块树的根位于名为crate的隐式模块下
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// 用mod关键字定义一个模块, 后面跟模块的名字, 并用{}包含模块的主体
mod front_of_house {  
    pub mod hosting {  // 使模块公有并不使其内容也是公有的,模块上的pub关键字只允许父模块引用它
        pub fn add_to_waitlist() {}  // 是hosting模块内的函数变为公有
        fn seat_at_table() {}
    }

    mod serving {   // 可以在模块中包含其他模块
                    // 模块中也可以包含其他项, 比如结构体、枚举、常量、trait、函数
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}


// 使用use和相对路径来将一个项引入作用域
// use crate::front_of_house::hosting;
// 使用use和相对路径来将一个项引入作用域
use front_of_house::hosting;  // 在作用域中增加use和路径, 类似于在文件系统中创建软链接
                              // 使用use关键字将路径一次性引入作用域
                              // 通过在crate根增加use front_of_house::hosting,现在hosting在作用域中
                              // 就是有效的名称了,如同hosting模块被定义于crate根一样。
                              // 此外,
                              // 通过use引入作用域的路径也会检查私有性,同其他路径一样,也就是说,要想
                              // 通过use引入一个模块, 前提是这个模块是公有的

// 使用use引入项时, 针对不同类型的惯用方法
// 1. 引入函数时: 
//      use front_of_house::hosting;  // 通常引入函数所在的父模块
// 2. 引入结构体、枚举、其他项时, 习惯是指定它们的完整路径
//      use std::collections::HashMap;   // 指定完整路径
//      fn main() {
//          let mut map = HashMap::new();
//          map.insert(1, 2);
//      }
// 3. 这个习惯用法有一个例外, 就是想使用use语句将两个具有相同名称的项带入作用域,方法如下:
//      // 如何将两个具有相同名称但不同父模块Result类型引入作用域, 以及如何引用它们
//      use std::fmt;
//      use std::io;
//
//      fn func1() -> fmt::Result {
//          ...
//      }
//
//      fn func2() -> io::Result {
//          ...
//      }

pub fn eat_at_restaurant() {
    // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    // 引入hosting模块后, 可以不再使用完整路径的方式来调用函数
    hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");  // meal.toast中原有的字符串内容被完全替换,因为String::from("Wheat")创建了新的
                                         // String对象, 此时原来的字符串内存会自动释放,当String类型的变量被另一个值覆盖
                                         // 时,原来的String会离开其作用域,rust的所有权规则确保不再使用的内存会被自动回收,
                                         // 这个过程称为drop
    println!("I'd like {}", meal.toast);
    // meal.seasonal_furit = String::from("apple");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


// 使用super起始的相对路径
fn server_order() {}

mod back_of_house {
    // 如果将枚举设为公有, 则它的所有成员都是公有的, 即枚举成员默认就是公有的
    pub enum Appetizer {  // 公有枚举
        Soup,       // 枚举的成员使用驼峰命名法
        Salad,
    }

    // 如果在结构体的定义前加了pub, 这个结构体会变成公有的, 但是结构体的字段仍然是私有的
    pub struct Breakfast {    // 结构体是共有的
        pub toast: String,      // 该字段公有
        seasonal_furit: String, // 该字段私有
    }

    // 定义Breakfast的方法
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_furit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::server_order();   // 使用super开头来构建从父模块开始的相对路径，这么做类似于文件系统中以..开头的语法
    }

    fn cook_order() {}
}

// self和super使用示例
mod network {
    fn connect() {}

    pub mod http {
        pub fn connect() {}

        pub fn perform_http_request() {
            // 使用self引用当前模块内的connect函数
            self::connect();

            // 使用super引用父模块network中的connect函数
            super::connect(); // super关键字是为了引用当前模块的父模块,因此super不能用在顶层模块中
        }
    }

}
