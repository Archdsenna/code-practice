fn main() {
    let s1 = String::from("hello");

    // let (s2, len) = caculate_length(s1);  // 当把变量传递到被调用的函数中时,该变量的所有权发生了转移,
                                          // 所以该变量不再有效,之后将不能再直接访问变量s1的值,只能通过引用的方式,
                                          // 通过引用的方式访问变量, 不会改变变量的所有权
                                          // rust的所有权和作用域规则:
                                          //    1. 当变量的所有权发生转移,其值将不再有效
                                          //    2. 当变量移出作用域后,其值将会通过`drop`被清理掉
                                          // 变量所有权发生转移,意味着该变量的值已经被移走(rust术语move),
                                          // 即该变量不再拥有对这个值的所有权(rust中的每一个值都有一个被称为
                                          // 其所有者的变量)
    let l = calculate_length(&s1);  // &s1创建了一个指向值s1的引用,但是并不拥有它。
                                    // 因为并不拥有这个值,所以当停止引用时,它指向的值也不会丢弃
    println!("s1 value is {}", s1);
    println!("s1 length is {}", l);

    let mut s2 = String::from("good");
    let s3 = change(&mut s2);
    println!("s3 is {}", s3);
}

// usize 的大小依赖于目标平台的架构。
// 在 32 位系统上，usize 是 32 位的；在 64 位系统上，usize 是 64 位的。
// 这使得 usize 在不同平台上能有效地利用其本地指针大小
// &表示使用对象的引用作为参数, 而不是获取值的所有权
// &符号就是引用, 它允许你使用值但不获得所有权
fn calculate_length(s: &String) -> usize {  // &表明参数s的类型是一个引用，即s是对String的引用
                                            // 创建一个引用的行为称为借用(borrowing)
                                            // &String表示对String类型的不可变引用,表示可以读取String的内容,但不能修改它
    // len()返回字符串的长度
    s.len()      // 注意这里不能有分号,因为这是一个表达式,表达式会返回值, 而语句不会返回值
}   // 这里, s离开了作用域。但是因为它并不拥有引用值的所有权
    // 所以什么也不会发生

// change函数将改变它所借用的值
fn change(s: &mut String) -> &str {    // &str是一个字符串切片类型
    s.push_str("hello");
    // 如果要返回值, 使用表达式, 表达式后不能加";"
    s.as_str() // as_str方法将String转换为&str类型的字符串切片
}
