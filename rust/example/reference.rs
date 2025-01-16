fn main() {
    let mut s = String::from("hello ");

    change(&mut s);
    change2(&mut s);
    println!("s is {}", s);

    // 可变引用的限制,在同一作用域内,只能有一个对某一特定数据的可变引用
    // 思考: 为什么会有这个限制?
    //       主要是为了确保引用的数据的唯一性和准确性,当存在多个可变引用时,任一可变引用都可随时修改数据,
    //       无法确保其他可变引用的数据内容。而只存在一个可变引用时, 在停止引用之前, 可确保该可变引用的
    //       数据不会发生变化,确保数据准确性
    // 这个限制的好处是: 在编译时就可以避免数据竞争。数据竞争的条件(以下三个需同时满足):
    //  1. 两个或更多指针同时访问同一数据
    //  2. 至少有一个指针被用来写入数据
    //  3. 没有数据同步访问的机制
    // 数据竞争会导致未定义行为, 难以在运行时追踪,并且难以诊断和修复,
    // rust避免了这种情况发生,rust甚至不会编译存在数据竞争的代码

    // 可以使用{}来创建一个新的作用域
    {
        let s1 = &mut s;
    } // s1在这里离开了作用域, 所以我们完全可以创建一个新的引用
    // println!("{}", s1);
    let s2 = &mut s;
    println!("exemple: {}", s2);  // 不可变引用s2的作用域在println!最后一次使用之后结束
                                  // 之后就可以创建可变引用


    let s3 = &s;  // 不可变引用没有限制,在同一作用域内,可创建多个不可变引用
    let s4 = &s;  
    println!("last: {} {}", s3, s4); // 一个引用的作用域从声明的地方开始,持续到
                                     // 最后一次使用位置

    // 注意, 我们既不能在拥有可变引用的同时,拥有不可变引用, 如下写法错误:
    // let sb = &mut s;
    // let sa = &s;
    // println!("new: {} {} {}", sa, sb);
    // 反之亦然, 不能在拥有不可变引用的同时,拥有可变引用, 如下写法错误: 
    // let sa = &s;
    // let sb = &mut s;
    // println!("new: {} {} {}", sa, sb);
    // 但是, 多个不可变引用是可以的
    // let sa = &s;
    // let sb = &s;
    // let sc = &s;

    // 悬垂引用
    let reference_to_nothing = dangle();
}

// main中语句
//  let reference_to_nothing = dangle();
// 报错如下:
//      此函数的返回类型包含借用的值，但没有可供借用的值
// 错误的dangle()函数
// fn dangle() -> &String {  // dangle()返回一个字符串的引用
//     let s = String::from("hello"); // s是一个新字符串
//     &s              // 返回字符串s的引用。这种写法的错误在于: s在函数结束时，其数据会被释放,
//                     // 所以返回指向这个数据的值的引用是一个无效的引用
// } // 这里, s离开作用域并被丢弃。 其内存被释放
  // 危险！
// 正确的dangle()函数
fn dangle() -> String {
    let s = String::from("hello");
    s   // 这样写没有错误, 把所有权移出去, 这样没有值被释放
}

fn change(s: &mut String) {
    s.push_str("world");
}

fn change2(s: &mut String) {
    s.push_str("world");
}
