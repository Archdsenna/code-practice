fn main() {
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();   // 清空String, clear()方法会将String的长度设置为0,但堆上的内容仍然被保留
                 // clear()方法需要获取一个可变引用来清空String
    let sa = "hello world";   // 这里sa是一个&str类型,&str是一个不可变引用,
                              // 字符串字面量是不可变的(加mut也不行)
                              // 字符串字面值就是字符串切片slice

    let mut my_str = String:from("hello world");

    // first_word接受String类型的切片,无论部分还是全部
    let word = first_word(&my_str[0..5]); // 部分
    let word = first_word(&my_str[..]);   // 全部

    // first_word也接受String类型的引用
    let word = first_word(&my_str);    // 这等同于`String`的全部切片

    let my_str_literal = "very good";
    // first_word接受字符串字面量的切片,无论部分还是全部
    let word = first_word(&my_str_literal[0..4]);
    let word = first_word(&my_str_literal[..]);

    // 因为字符串字面量就是字符串slice, 这样写也可以,即不使用slice语法
    let word = first_word(my_str_literal);
}

fn first_word(s: &str) -> &str { // &str是字符串切片类型
    // 在 Rust 中，字符串通常以 UTF-8 格式存储，这意味着每个 Unicode 字符可能由一个或多个字节表示。
    // 使用 as_bytes() 方法可以获取字符串的底层字节表示，这对于基于字节的操作（如查找特定字节或处理 ASCII 字符）非常有用
    let bytes = s.as_bytes();      // 将String转换为一个字节数组
                                   // s是一个对String的引用,as_bytes返回一个字节数组的切片(&[u8])

    // 使用rust的迭代器方法来遍历字节数组,并同时获取每个元素的索引和值
    for (i, &item) in bytes.iter().enumerate() {  // 使用iter方法在字节数组上创建一个迭代器
                                                  // iter方法返回集合中的每一个元素
                                                  // 而enumerate包装了iter的结果,将这些元素作为元组的一部分返回
                                                  // enumerate返回的元组中,第一个元素是索引,第二个元素是集合中元素的引用
                                                  // 使用模式来解构enumerate的结果
        if item == b' ' {
            return &s[0..i]; // 返回字符串切片
        }
    }

    &s[..];   // 返回字符串切片,该切片包含整个字符串
}

