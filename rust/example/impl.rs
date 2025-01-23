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
