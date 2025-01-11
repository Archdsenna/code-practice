// for word in words {
//     if !key_add.contains(word) {  // rust要求控制流语句(如if)后必须跟一个代码块,即使只有一条语句也需要用{}括号包围
//     }
// }

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];
    let mut vec3 = vec![1, 2, 3];

    // 总结: for与迭代器的三种使用方法
    //  1. for item in vec1: 这会隐式调用.into_iter()方法,该方法返回一个迭代器,同时迭代器会获取vec1变量的所有权,
    //                       该迭代器的next()方法在每次迭代时返回元素的所有权
    //  2. for item in vec1.iter(): .iter()方法同样返回一个迭代器,迭代器不会获取vec1的所有权,
    //                               迭代器的next()方法每次返回元素的`不可变引用`
    //  3. for item in vec1.iter_mut(): iter()方法同样返回一个迭代器,迭代器不会获取vec1的所有权,
    //                               迭代器的next()方法每次返回元素的`可变引用 `
    
    // Q: for 循环会自动解引用吗?
    // A: 如果使用了模式匹配,则for循环会自动解引用, 例如:
    //      for &item in vec1.iter() // for循环会自动解引用&item, 此时item的类型是i32
    //      for item in vec1.iter()  // 此时for循环不会自动解引用, item是&i32类型,如果要访问item指向的内容,需要手动解引用:*item

    // 获取元素的所有权
    for item in vec1 {     // 这个语句会隐式调用vec1.into_iter()方法,这个方法会消耗原始的集合,
                           // vec1的所有权被转移到了由.into_iter创建的迭代器中,这意味着vec1的所有权被迭代器获取了
                           // 因此在for循环之后,vec1不再有效,不能再被访问或有效
                           // 迭代器的行为: 在for循环中, 迭代器的next()方法被重复调用,每次调用都返回集合下一个元素的所有权,
                           // 一旦所有元素都被遍历完,迭代器返回None,这标志着迭代结束
        println!("vec1: {}", item);
    }
    // println!("vec1: {:?}", vec1);   // vec1所有权发生了转移，再次访问会出错

    // 获取元素的不可变引用
    for item in vec2.iter() { // 不会获取元素的所有权,iter()会返回每个元素的不可变引用
                              // vec1.iter()会返回一个迭代器,这个迭代器的next()方法将返回向量vec中每个元素的不可变引用
        println!("vec2: {}", item);  // item是一个&i32类型,但是println宏会自动解引用
    }

    // 获取元素的可变引用
    for item in vec3.iter_mut() {
        *item += 1;
    }
    println!("vec3: {:?}", vec3);  // Vec<T>类型没有实现Display的trait,但有Debug的trait,所以可以使用调试格式打印
}
