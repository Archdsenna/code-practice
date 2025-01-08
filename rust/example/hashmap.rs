use std::collections::HashMap;   // 因为HashMap不是特别常用,所以没有被prelude自动引用
                                 // 但是String类型很常用,因此会被自动引用
// Q: 什么是rust的prelude自动引用
// A: 在Rust中，prelude 是一个特殊的模块，它包含了一组被自动引入到每个Rust程序的类型和特性。
//    这意味着你可以在任何 Rust 程序中使用这些类型和特性，而无需显式地使用 use 语句来引入它们。
//    这样做的目的是为了减少常用类型和特性的引用代码，使得 Rust 程序更加简洁易读。

// Rust Prelude 的内容
//      Rust 的标准库中的 std::prelude 模块包括了几个不同的版本，其中最常用的是 std::prelude::v1。
//      以下是一些 std::prelude::v1 中包含的重要类型和特性：
//      (1) 核心类型：
//          Option 和 Result：这两个枚举类型用于错误处理和可选值处理。
//          String 和 Vec：常用的集合类型。
//      (2) 特性：
//          Copy、Clone：用于控制对象的复制行为。
//          Debug：用于格式化输出，特别是用于调试目的。
//          Default：为类型提供默认值的能力。
//          Drop：用于在值离开作用域时执行清理代码。
//          Eq、PartialEq、Ord、PartialOrd：用于比较操作。
//          Hash：用于支持哈希操作。
//      (3) 宏：
//          println! 和 format!：用于输出和字符串格式化。
//          vec!：用于快速创建向量。
//      (4) 函数：
//          drop：显式地释放给定的值。


fn main() {
    // 新建一个hashmap, 存储一个键值对,键为队名, 值为队的得分
    let mut scores = HashMap::new();
    
    // 向HashMap中插入键值对
    scores.insert(String::from("Blue"), 99);    // HashMap的键类型是String，值类型是i32
    scores.insert(String::from("Yellow"), 98);  // HashMap所有的值必须是相同类型,值也必须是相同类型
    for val in &scores {
        println!("first scores: {:?}", val);  // 只有实现了Display的类型才能使用{}格式直接输出,
    }                                         // 不是所有类型都实现了Display,但是所有类型都应该实现Debug,
                                              // 实现了Debug的类型,可以使用{:?}或者{:#?}格式化打印
    // 使用get方法访问hashmap中的值
    let team_name = "Blue";
    let score_of_blue = scores.get(team_name);  // get方法返回一个Option<T>类型
    match score_of_blue {
        Some(score) => println!("score of blue: {:?}", score),
        None => (),
    }

    // 使用for循环打印HashMap
    for (key, value) in &scores {  // 这会以任意的顺序打印键值对
        println!("key: {}, val: {}", key, value);
    }

    // 覆盖HashMap中的值
    scores.insert(String::from("Black"), 100); 
    scores.insert(String::from("Black"), 96);  // 这次插入会覆盖上一次的值
    println!("black value: {:#?}", scores);

    // 只在键没有对应值的时候插入,HashMap有一个特有的API, 叫entry
    scores.entry(String::from("Pink")).or_insert(100);  // 只在键没有对应值时插入
    println!("after insert pink scores: {:#?}", scores);

    // 根据旧值更新一个值
    // 例子: 统计文本中每个单词出现的次数
    let text = "hello world hello world good hello very";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let mut count = map.entry(word).or_insert(0); // or_insert方法会返回这个键的值的可变引用(&mut V)
                                      // 如果key没有值,则插入0
                                      // entry()把想要检查的键作为参数
                                      // entry()的值是一个枚举Entry, 代表可能存在也可能不存在的值
                                      // Entry的or_insert方法在键对应的值存在时,就返回这个值可变引用,
                                      // 如果键对应的值不存在, 则将参数作为新值插入,并返回新值的可变引用
        *count += 1;  // 使用*操作符来解引用, 以操作引用指向的数据
                      // 这个可变引用在for循环的结尾离开作用域
    }
    println!("map value is : {:#?}", map);

    // 另一个创建HashMap的方法,使用一个元组的vector的collect方法
    let teams = vec![String::from("Red"), String::from("Green")];
    let init_scores = vec![99, 96];

    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
    // 代码分析:
    //  在 Rust 中，teams.iter().zip(initial_scores.iter()).collect(); 这行代码是一个非常典型的例子，
    //  展示了 Rust 如何使用迭代器、适配器和收集器来构建复杂的数据结构。
    // 分解代码:
    //  1. 迭代器创建
    //      teams.iter(): 这会为teams向量创建一个迭代器,该迭代器会逐一产生teams中的元素的不可变引用,
    //                    即&String类型
    //      init_scores.iter(): 这会为init_scores创建一个迭代器,该迭代器会逐一产生init_scores向量中的元素的不可变引用,
    //                    即&i32类型
    //  2. 迭代器适配器zip:
    //      .zip(init_scores.iter()): zip是一个迭代器适配器,它接受另一个迭代器作为参数,并将两个迭代器中的元素配对,
    //                                结果是一个新的迭代器, 每次迭代时产生一对元素(一个来自teams, 另一个来自init_scores),
    //                                因此每个元素都是一个元组,形式为(&String, &i32)
    //      zip名字的含义: 在Rust中,迭代器适配器zip的命名来源于其功能的直观描述: 它将两个迭代器中的元素“拉链式”的配对
    //                     起来,这个名称是从日常生活中的"拉链(zipper)"借鉴而来,因为拉链的工作方式是将两排扣眼交替连接起来,
    //                     形成一个连续的封闭结构
    // 3. 收集器collect
    //      .collect(): 这是一个非常强大的方法,它能从迭代器中消费所有元素,并将它们收集到某种集合中。
    //                  在这个例子中,collect方法被用来将元组的迭代器转换成一个HashMap, 这里的类型注解表明HashMap<_, _>
    //                  表明我们希望collect方法生成一个HashMap。编译器会根据上下文(即zip生成的元组类型)推断出具体的键值类型
    for val in &scores {
        println!("second scores: {:?}", val);
    }

    // HashMap的所有权
    let fruit1 = String::from("apple");
    let fruit2 = String::from("banana");
    let mut fruit_hash = HashMap::new();
    // fruit_hash.insert(fruit1, fruit2);   // fruit1和fruit2发生了所有权转移,此时fruit1和fruit2不再有效,后面继续访问会
                                         // 导致悬垂访问
    fruit_hash.insert(fruit1.clone(), fruit2.clone());
    println!("fruit: {} {}", fruit1, fruit2);  // 报错: value borrowed here after move,变量在移动之后又被借用
}
