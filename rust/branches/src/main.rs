// branches
fn main() {
    let number = 6;

    if number < 8 {   // if语句的条件必须是bool值，错误写法：if number,rust不会自动把非bool值转换为bool值
                      // 必须自始至终显式地使用bool值作为if的条件
        println!("hello");
    } else {
        println!("world")
    }

    let mut num = 99;
    if num > 100 {   // if是一个表达式
        num = num -1;
    } else if num > 90 {
        num = num + 1;
    }
    println!("num is {}", num);

    let condition = true;
    let _n = if condition { 6 } else { 9 }; // 因为if是一个表达式,我们可以在let语句的右侧使用它来将结果赋值给一个变量
}


