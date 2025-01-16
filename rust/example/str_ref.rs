fn main() {
    let s1 = String::from("hello");
    let r1 = &s1;
    println!("s1 value: {}", r1);   // 引用r1指向存在的数据

    let s2 = s1;  // s1所有权转移, s1失去了对数据的所有权, s2拥有数据的所有权
                  // 此时s1的引用r1事实上已经成为了一个悬垂引用, 因为此时它指向的数据已经不存在了
    println!("s1 value: {}", r1);   // 再次访问引用r1将会报错
}
