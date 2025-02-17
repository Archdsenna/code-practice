// 打印单元结构体变量的值

#[derive(Debug)]
struct UnitStruct;

#[derive(Debug)]
struct TupleStruct(
    i32,
    i32,
    i32,
);

#[derive(Debug)]
struct NormalStruct {
    name: String,
    age: i32,
}

fn main() {
    // 实例化一个单元结构体
    // `println!`宏中的变量名直接放在花括号里，这是Rust 1.58以上版本支持的写法，
    // 可以省略后面的参数列表，直接捕获上下文中的变量。
    // 比如`println!("{unitstruct:?}")`等价于`println!("{:?}", unitstruct)`
    let unitstruct = UnitStruct;
    println!("I am {unitstruct:?}");
    // 在Rust中，`{}`用于显示格式化，而`{:?}`是用于调试格式化的
    // {}需要实现Display trait, {:?}需要实现Debug trait

    let tuplestruct = TupleStruct(0, 0, 0);
    println!("This is {tuplestruct:?}");

    let student = NormalStruct {
        name: String::from("david"),
        age: 18,
    };
    println!("The is student is {student:#?}");
}
