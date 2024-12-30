// loops
fn main() {
    let mut count = 0;
    // loop会无限循环
    'counting_up: loop {     // 定义一个循环标签
        println!("count = {}", count);
        let mut remaining = 10; // 定义一个可变的变量,rust中变量默认不可变

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;   // 终止内层循环
            }
            if count == 2 {
                break 'counting_up;  // 跳出循环标签所在的循环体
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count is {}", count);

    // 循环终止时返回值: break number;
    let mut counter = 0;
    let result: u32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 使用break在循环终止时返回值
        }
    };
    println!("result value is {}", result);

    // while 循环
    let mut number = 3;

    while number != 0 {
        println!("number is {}", number);
        number -= 1;
    }

    // 使用while打印数组元素
    let array: [u32; 5] = [1, 2, 3, 4, 5];
    let arr_len = array.len();
    let mut index = 0;

    while index < arr_len {
        println!("arry[{}]: {}", index, array[index]);
        index += 1;
    }

    // for循环:对一个集合的每个元素执行一些代码
    let array_a = [6; 8];
    for elem in array_a { // 在这个for循环中,elem是自动定义的循环变量
        println!("elem is {}", elem); // 每次迭代,自动变量会被赋值为当前元素的值
    }

    // 使用for循环控制循环次数
    for i in 1..4 {   // ..是rust的范围语法,表示前闭后开,如果要包含结束值,则用..= 
    // for i in 1..=4 {   
        println!("current number is {}", i);
    }

    // 1..4中每一个数字是一个迭代器
    // 在 Rust 中，范围（如 (1..4)）实现了 Iterator trait，这意味着它们提供了一系列与迭代器相关的方法，包括 rev()。
    // rev() 方法是 Iterator trait 的一部分，它用于反转迭代器的顺序。
    for i in (1..=4).rev() { // i可重复使用,i是for循环创建的自动变量,生命周期/作用域仅在for循环体中
        println!("number is {}", i);
    }
}
