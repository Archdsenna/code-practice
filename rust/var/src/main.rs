fn main() {
    let mut y = 5;
    println!("The value of y is {}", y);
    y = 6; // 使用这种方式必须使用mut进行修饰,表明变量可变,rust中变量默认不可变
           // 但注意，此种方式不能更改变量的类型，如果要同时改变变量的类型，要使用变量遮蔽
    println!("The value of y is {}", y);

    // 变量遮蔽
    let x = 7;
    println!("The value of x is {}", x);
    let x = 8;
    println!("The value of x is {}", x);

    //创建新的作用域
    {
        // 新的作用域内，允许定义新的局部变量，这些变量只在这个作用域内可见
        // 当作用域结束，所有的局部变量全部会被销毁
        let x = 9; 
        println!("The new domain x value is {}", x);
    }

    let z = {
        let x = 6;
        x + 1       // x + // 1为表达式,而不是语句,语句不会返回值,而表达式会返回值。表达式的末尾没有分号
    };
    
    println!("The value of z is {}", z);

    // 变量定义之后，必须有明确的访问，否则会编译警告,下面的写法是有问题的:
    // let x = 5;  此处的变量x没有访问，会编译告警
    // let x = 6;
    // println!("value of x is {}", x);

    const THREE_NUM_ADD: u32 = 10 + 11; // rust中的常量必须大写,单词之间用下划线分隔,并且进行类型注解
    println!("The THREE_NUM_ADD value is {}", THREE_NUM_ADD);
}
