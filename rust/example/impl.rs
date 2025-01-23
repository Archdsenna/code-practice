// impl的参数
// 在Rust中,使用impl关键字时,是否需要参数取决于你正在实现的内容。
// 具体来说,这里的"参数"通常指的是泛型参数或特征约束
//
// 下面是几种不同情况下impl的使用:
//  1. 普通实现(无参数)
//      当你为一个非泛型的结构体或枚举实现方法时, impl后不需要跟任何参数
//          struct Point {
//              x: i32,
//              y: i32,
//          }
//
//          impl Point {    // 在这个例子中,impl Point后没有任何参数,因为Point是个具体的、非泛型的类型
//              fn new(x: i32, y: i32) -> Self {
//                  Point { x, y }
//              }
//
//              fn details(&self) {
//                  println!("Point({}, {})", self.x, self.y);
//              }
//          }
//
// 2. 泛型实现
//      如果你为一个泛型结构体实现方法,你需要在impl后指定泛型参数
//      struct Point<T> {
//          x: T,
//          y: T,
//      }
//      
//      impl<T> Point<T> {    // 表示这个实现块适用于所有类型T的Point。T是一个泛型参数,需要在impl后指出
//                            // 如果这里的impl后没有声明<T>,那在impl的实现块中使用T时,会提示找不到类型T
//          fn new(x: T y: T) -> Self {
//              Point { x, y }
//          }
//      }
// 3. 特征实现
//      trait Printable {
//          fn print(&self);
//      }
//
//      impl<T> Printable for Point<T> where T: std::fmt::Display {
//          fn print(&self) {
//              println!("...")
//          }
//      }

struct Point<T> {
    x: T,
    y: T,
}

// 为泛型Point实现方法
// impl<T> Point<T> {          // 在impl后声明泛型T,这样Rust就知道Point的尖括号中的类型是泛型而不是具体类型了
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// 为泛型Point<i32>实现方法, 而不是泛型Point
impl Point<i32> {           // 如果不在impl后声明<T>,表示不涉及泛型
    fn x(&self) -> &i32 {
        &self.x
    }
}

fn main() {
    let p = Point {
        x: 8,
        y: 9,
    };

    println!("value of x: {}", p.x());
}
