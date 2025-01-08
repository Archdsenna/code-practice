// rust每种类型定义方法的简要说明和示例

// 结构体struct 
// 结构体是rust中最常用来定义方法的类型
struct Point {   // rust结构体的命名通常遵循驼峰命名法(CamelCase),即每个单词的首字母都大写
    x: i32,
    y: i32,
}

// 定义结构体的方法
impl Point {   // 方法在结构体的实现块中定义
    fn distance_to_origin(&self) ->i32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


/// 枚举类型
/// 枚举中的成员称为枚举变体,枚举的变体可以看作是该枚举类型的一个具体的实例化形式,它代表了枚举可能得值之一
/// 枚举变体的不同形式:
///     1. 无数据变体:
///        如Quit, 它不携带任何额外的数据
///     2. 元组风格变体
///        如Write(String)，它类似于元组,可以包含一个或多个类型的值
///     3. 结构体风格变体
///        如Move {x: i32, y: i32},它使用{}，并像结构体一样有命名字段
enum Message {
    Quit,                   
    Move {x: i32, y: i32},
    Write(String),
}

// 定义枚举类型的方法
impl Message {
    fn call(&self) {
        match self {  // rust模式匹配
            Message::Quit => {  // Message枚举的变体
                println!("Quit no data");
            },
            Message::Move(x, y) => {  // ::是一个作用域解析运算符,用于指定枚举、模块或特征中的具体项
                                      // ::的作用:
                                      //    1. 访问枚举的成员
                                      //    2. 访问关联函数
                                      //    3. 模块和其内部函数或类型: 如果您有一个模块包含函数或类型,
                                      //       同样使用::来访问,例如，如果有一个名为math的模块,里面有一个
                                      //       函数add, 则使用math::add()来调用
                println!("move to x:{} y:{}", x, y);
            },
            // 当match语句运行到这个分支时,它会自动解构Write变体,提取里面的String,并将这个字符串绑定到变量text上
            Message::Write(text) => {  // 用来匹配枚举Message中的Write变体,并将其包含的数据绑定到变量text上
                                       // 这种写法是rust模式匹配的一部分
                println!("text is {}", text);
            }
        }
    }
}


// 特征trait
// trait定义了一组方法,这些方法可以被任何类型实现
trait Drawable {
    // 定义了一个draw的方法
    fn draw(&self);   // 注意这里是分号;
}

struct Circle {
    radius: f64,
}

// 所有在impl块中定义的函数被称为关联函数, 因为它们与后面命令的类型有关
impl Drawable for Circle {
    fn draw(&self) {
        println!("hello");
    }
}
