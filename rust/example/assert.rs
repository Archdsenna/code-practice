// assert：断言的意思
// assert!宏的作用: assert!是一种在运行时检查条件是否为真的调试工具
//  如果传递给assert!的条件表达式的结果为true,则程序继续执行
//  如果条件表达式结果为false, 则assert!会导致程序立即终止,并显示一条错误消息
//
// 使用assert!宏的基本语法:
//  assert!(expression)
// 自定义错误消息:
//  assert!(expression, "Custom error message: {}", info);


fn main() {
    let equal = |x, y| x == y;
    // let a = 5;
    let a = 6;
    let b = 6;
    assert!(equal(a, b), "{} equal {}", a, b);  // assert!宏会根据表达式的值决定是否中断程序执行
    println!("hello");
}
