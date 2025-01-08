// lib.rs就是crate的根, 所以在这里定义的都是处于根层级
// src/
// ├── lib.rs
// ├── front_of_house.rs
// └── back_of_house.rs

// 这里的mod front_of_house;和mod back_of_house;告诉rust编译器在当前库的根目录下,
// 查找名为front_of_house.rs和back_of_house.rs的文件。这些文件应该包含与模块名对应的代码,
// 通过这种方式,lib.rs成为了这些模块的父模块
mod front_of_house;// 告诉rust,在另一个与模块同名的文件中加载模块内容,即将同名模块的内容加载进来,rust会在与模块同名的文件中查找模块的代码
mod back_of_house; // front_of_house.rs和back_of_house.rs这些文件中定义的模块可以进一步的
                   // 包含模块化的内容,如子模块、函数、结构体等。
                   // 在这些文件中定义的公开(pub)内容, 可以通过在lib.rs中使用pub use语句重新导出,
                   // 使得这些功能可以被库的用户直接访问,而不需要知道具体的模块路径

// 使用pub use将add_to_waitlist和fix_order函数重新导出,
// 这意味着外部代码可以直接使用这些函数,而不需要通过各自的模块路径, 例如:
//      extern pub-use
//
//      fn main() {
//          pub-use::add_to_waitlist();
//          pub-use::fix_order();
//      }
pub use crate::back_of_house::cooking::fix_order;
pub use crate::front_of_house::hosting::add_to_waitlist;

fn eat_at_restaurant() {
    fix_order();
    add_to_waitlist();
}
