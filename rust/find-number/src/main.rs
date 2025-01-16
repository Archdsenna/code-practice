// 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、
// 中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
// 中位数：如果有偶数个数, 取中间两个数的平均数作为中位数

use std::collections::HashMap;
use std::io;

fn main() {
    // 接收用户输入的数字列表
    let mut list = String::new();

    println!("Please input number list such as 1 2 3 4 5 ...");
    io::stdin().read_line(&mut list).expect("input error");
    println!("input: {}", list);

    let mut list_vec: Vec<i32> = Vec::new();
    let mut list_hash: HashMap<i32, i32> = HashMap::new();
    let avg: i32;   // 在后面明确使用之前，这里可以先不用赋初始值
    let median;
    let mut sum = 0;
    let list_len = list.chars().filter(|c| !c.is_whitespace()).count();  // 闭包用法
                                       // |c| !c.is_whitespace()整个称为一个闭包, 其中,
                                       // c是闭包的参数, 闭包的参数通过||提供, ||类似于函数的()
                                       // filter是一个过滤器,用于根据给定的条件(通过一个闭包提供)筛选出元素
                                       // filter方法接受一个闭包,闭包的参数通过管道符'|'来定义
    for num in list.split_whitespace() {  // split_whitespace按空白字符(空格、制表符、换行符等)分割字符串,
                                                // 并返回一个迭代器,该迭代器会逐项产生字符串中的非空空白子字符串
        // for average
        let n: i32 = num.parse().unwrap();  // unwrap意为"拆包",即从Option<T>类型或者Result类型中将值解包出来,
                                            // 如果parse失败, unwrap会使程序panic并报告错误信息,但是unwrap_of不会panic,
                                            // 同时返回给定的值
        sum += n;

        // for median
        list_vec.push(n);

        // for mode
        let count = list_hash.entry(n).or_insert(0);
        *count += 1;
    }

    // 计算平均数
    let len = list_len as i32;
    // println!("len: {}", len);
    avg = sum / len;
    dbg!(avg);   // dbg宏的输出: [src/main.rs:42:5] avg = 4
    println!("list avg: {}", avg);

    // 计算中位数
    let i = list_len / 2;
    if list_len % 2 == 0 {
        median = (list_vec[i - 1] + list_vec[i]) / 2;
    } else {
        median = list_vec[i];
    }
    println!("list median: {}", median);

    // 计算众数
    let mut max_val = 0;
    let mut max_key: i32 = 0;
    for (&key, &val) in list_hash.iter() { // 迭代器返回的是key和value的引用,
                                                     // &val的写法是为了解引用
                                                     // 使用模式匹配解引用 (&value) 的好处是可以直接得到值的拷贝
                                         //如果值类型实现了Copy trait,或者直接以值的形式使用,而不需要在循环体内手动解引用
        if val >= max_val {
            max_val = val;
            max_key = key;
            // println!("max_key: {}, max_val: {}", max_key, max_val);
        }
        // println!("key: {} val: {}", key, val);
    }
    let mode = max_key;
    println!("list max: {:?}", mode);
    
    // 使用vector返回三种数
}
