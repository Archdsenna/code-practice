// trait: 定义共享的行为

// trait体中可以有多个方法: 一行一个方法签名且都以分号结尾
trait Summary {
    // 方法签名后跟分号
    fn summarize(&self) -> String;   // 实现这个trait的类型所需要的行为的方法签名
                                     // 每一个实现这个trait的类型,都需要提供其自定义行为的方法体
}

// 实现trait时需要注意的限制: 只有当trait或者要实现trait的类型位于crate的本地作用域时,才能为该类型实现trait,
// 但是不能为外部trait实现外部trait
pub trait Message {
    fn get_message(&self) -> String;
}

struct Teacher {
    name: String,
    years: i32,
}

impl Message for Teacher {
    fn get_message(&self) -> String {
        format!("{} {}", self.name, self.years);
    }
}

struct Student {
    name: String,
    class_id: i32,
}

impl Message for Student {   // Student是实现了Message trait的类型
    fn get_message(&self) -> String {
        format!("{} {}", self.name, self.class_id);
    }
}

fn main() {

}

// trait的默认方法
pub trait MessageDefault {
    fn get_message(&self) -> String {
        String::from("Default transport...")
    }
}

struct Chef {
    val: None,
}

impl MessageDefault for Chef {}     // 指定一个空的impl块,表示使用默认的trait方法

// trait作为参数, 只有一个参数
fn notify(item: impl MessageDefault) {
    println!("{}", item.get_message());
}

// trait作为参数的第二种写法: trait bound
fn notify1<T: MessageDefault>(item: T) {
    println!("{}", item.get_message());
}

// trait作为参数, 当有多个参数(多个不同的类型)
fn notify2(item1: impl MessageDefault, item2: impl MessageDefault) { // item1和item2可以是两个实现了MessageDefault的不同类型
    println!("{} {}", item1.get_message(), item2.get_message());
}

// trait作为参数, 当有多个参数(多个,但类型相同),可以使用trait bound语法
fn notify3<T: MessageDefault>(item1: T, item2: T) {    // item1和item2值的类型必须一致
    println!("{} {}", item1.get_message(), item2.get_message());
}
// 通过+指定多个trait bound
fn notify4(item: impl MessageDefault + Display) {}   // item具有2个不同的trait
fn notify5<T: MessageDefault + Display>(item: T) {}  // +语法也适用trait bound, 指定了2个trait bound

// 当有多个泛型类型，并且每个泛型也都有多个trait
fn notify6<T: Display + Clone, U: Clone + Debug>(item1: T, item2: U) -> i32 {} 
// 上面的签名可以使用where语句改写
fn notify7<T, U>(item1: T, item2: U) -> i32 
where T: Display + Clone,
      U: Clone + Debug
{
    // ...
}

// trait作为返回值
// 在返回值中使用impl trait语法,来返回某个实现了trait的类型
fn ret_func() -> impl Message { // 函数返回某个实现了Message trait的类型,但不确定具体的类型,在闭包和迭代器场景十分有用
    Teacher {                   // 闭包和迭代器创建只有编译器知道的类型,或者是非常非常长的类型,
                                // impl Trait允许你简单的指定函数返回一个Iterator(迭代器)而无需写出实际的冗长的类型
        name: String::from("david"),
        years: 29,
    }
}   

// 使用trait bound来有条件地实现方法
use std::fmt::Display

struct Pair<T> {
    x: T,
    y: T,
}

// 对所有Pair<T>类型总是实现了new方法
impl<T> Pair<T> {    
    fn new(x: T, y: T) -> Self {   // Self关键字是一个类型别名,它代表当前实现块(impl)的类型,此处Self代表的是Pair<T>类型,
                                   // Self好处,使用Self可以简化代码,特别是在类型名较长或复杂时,或者在类型名称可能会改变时,
                                   // 使用Self可以避免重复和减少需要修改的地方
        Self {
            x,    // 当变量与结构体内的字段名字相同时,可以直接写变量名,不用:的写法
            y,
        }
    }
}

// 只有为那些T实现了Display trait和PartialOrd trait的Pair<T>类型才会实现cmd_display方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("largest is : {}", self.x);
        } else {
            println!("largest is : {}", self.y);
        }
    }
}

