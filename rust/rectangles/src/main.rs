// 计算长方形的面积

fn main() {
    // 版本1，初始版本
    // let width1 = 30;
    // let height1 = 50;
    // println!("area is {}", area(width1, height1));

    // 版本2，使用元组
    // let rect: (i32, i32) = (30, 50);
    // println!("area is {}", area(rect));

    // 版本3，使用结构体
    let rect = Rectangle { // 创建一个结构体实例
        width: 30,
        height: 50,
    };

    // println!("area is {}", area(&rect));
    println!("rect value is {}", rect.area());  // 改成使用方法语法, 在Rectangle实例上调用area方法
    println!("debug output: {:?}", rect);  // :?指示符告诉println!我们要使用叫做debug的输出格式
    println!("debug output: {:#?}",rect);  // {:#?}是一种特殊的格式化字符串,用于在打印调试信息时提供更易读的输出
                                            // 这种格式化通常用于Debug trait的输出,特别是当输出结构体或枚举等复杂
                                            // 数据类型时。
                                            // 其中,?标记指示rust使用变量的Debug实现来格式化输出,Debug是一个trait,它允许类型以
                                            // 调试友好的方式显示。对于许多复杂类型,rust无法直接使用{}来打印。因为不是所有类型
                                            // 都实现了Display trait。但几乎所有的类型都应该实现Debug trait。
                                            // 
                                            // #：这个符号是一个格式化标志，用于修改 Debug 输出的样式。当使用 # 时，
                                            // 输出会变得更加“美观”或“结构化”。对于多行数据结构，如列表、哈希表或复杂的嵌套结构，
                                            // 这会使每个元素或字段都独立一行，并且更加缩进，从而提高可读性
                                            // println! 宏使用的是 {:?} 或 {:#?} 格式化字符串，这要求被打印的类型实现了 
                                            // std::fmt::Debug 特征。这种情况下，通常只需要一个对数据的不可变引用
    // 使用dbg!宏打印
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);   // 不要让dbg获得rect2结构体实例数据的所有权


    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 60,
    };

    println!("Can rect hold rect1: {}", rect.can_hold(&rect1));
    println!("Can rect hold rect2: {}", rect.can_hold(&rect2));

    // 调用Rectangle类型的关联函数,创建一个新的实例
    let rect3 = Rectangle::square(6);  // 使用::语法关联命名空间
                                                        // 在rust中,::语法用于访问与特定类型或模块相关联的命名空间
                                                        // 例如Rectangle::square(),表示square属于Rectangle类型的命名空间
    println!("rect3 width: {}, heitht: {}", rect3.width, rect3.height);

}

// 版本1，初始版本
// fn area(height: i32, width: i32) -> i32 {
//     height * width
// }

// 版本2,使用元组重构
// fn area(rect: (i32, i32)) -> i32 {
//     rect.0 * rect.1
// }

// 版本3，使用结构体
// 给Rectangle类型增加Debug trait
// 告诉编译器自动实现Debug特征的实现代码
#[derive(Debug)]  // derive为派生,增加属性来派生Debug trait,并使用调试格式打印Rectangle实例
struct Rectangle {
    width: i32,
    height: i32,
}

// 为结构体定义方法
// 方法在该结构体的实现块(impl)中
// impl块,impl(implementation, 实施、执行)
// 所有在impl块中定义的函数被称为关联函数
impl Rectangle {   // 为了使函数定义于Rectangle的上下文中,我们开始了一个impl块。这个impl块中的所有内容都将与Rectangle类型相关联
    // rust中，方法的第一个参数通常是 self、&self、&mut self
    //      1. &self(仅读取):   表示方法通过不可变引用访问调用它的实例, 这允许方法读取实例的数据但不能修改它
    //      2. &mut self(做出修改): 表示方法通过可变引用访问调用它的实例, 这允许方法修改实例的数据
    //      3. self(获取所有权):    表示方法获取调用它的实例的所有权。使用这种形式时,调用方法后,原始
    //                  实例将不再可用,除非方法返回了这个实例
    // Q: 方法定义时,第一个参数必须使用self吗？
    // A: 在定义方法时,第一个参数名必须是self、&self、&mut self, 这是语法要求, 这样设计是为了在语言层面上
    //    普通函数和方法。
    //    如果不使用self、&self、&mut self作为第一个参数,那么该函数将不再是一个方法, 而是成为了一个关联函数,
    //    关联函数是定义在结构体、枚举或trait的命令空间内的函数,但它们不取任何形式的self参数,因此它们不作用
    //    于结构体的一个具体实例。关联函数通常通过类型调用, 而不是通过一个实例调用。
    //
    //    代码实例如下:
    //      struct Point {
    //          x: i32,
    //          y: i32,
    //      }
    //
    //      impl Point {
    //          // 定义一个方法
    //          fn clear(&mut self) {
    //              self.x = 0,
    //              self.y = 0,
    //          }
    //
    //          // 这是一个关联函数
    //          fn origin() -> Point {
    //              Point {
    //                  x: 0,
    //                  y: 0,
    //              }
    //          }
    //      }
    //
    //      fn main() {
    //          let mut p = Point {
    //              x: 9,
    //              y: 9,
    //          };
    //
    //          // 调用方法
    //          p.clear();   // 修改了实例
    //          
    //          // 调用关联函数
    //          let origin = Point::origin();  // 不操作实例,而是返回一个实例
    //      }
    //    
    fn area(&self) -> i32 {   // 将签名中的第一个参数和函数体中其他地方的对应参数改成self。
                              // &self是self: &Self的缩写
                              // 如果想要改变调用方法的实例,则改为&mut self
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) ->bool {
        self.width > other.width && self.height > other.height
    }
} // 使用方法替代函数的好处,将某个类型实例能做的所有事情都一起放入impl块中

// fn area(rect: &Rectangle) -> i32 {  // 参数为Rectangle结构体的不可变引用,因为我们希望使用结构体,而不是获取它的所有权
//     rect.width * rect.height
// }

// 定义一个Rectangle关联函数,创建一个新的Rectangle实例
impl Rectangle {
    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
