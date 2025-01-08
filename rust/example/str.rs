fn main() {
    let mut s = String::from("hello");
    let s1 = "world";
    s.push_str(s1);  // push_str不会获取s1的所有权
                     // push_str用于拼接一个新的字符串到原字符串末尾
    s.push('6');  // push用于拼接一个字符到原字符串
    // println!("the value of s1 is {}", s1);
    println!("the value of s is {}", s);

    let a = 32; // 这里的a是i32类型, 它实现了Copy trait,因此当a赋值给b时,a的值被复制到b,并且a仍然保持有效和独立
                // 基本的数值类型(如整数和浮点数),布尔类型,字符类型以及这些类型的固定大小数组都实现了Copy trait
    let b = a;  
    println!("a value is {}", a);
    println!("a value is {}", a);

    // 字符串拼接
    let n1 = "hello";
    let n2 = "good";
    let mut n = format!("{}-{}", n1, n2);  // format宏会返回一个带有结果内容的String
    n.push_str("verry");
    println!("value of n is {}", n);
}
