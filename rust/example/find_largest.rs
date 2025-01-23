fn main() {
    let nums = vec![10, 99, 68, 29, 66];
    // let nums = Vec::new();
    let max = get_max(&nums);
    println!("max is {}", max);
}

// fn get_max(num_vec: &Vec<i32>) -> i32 {  // &Vec<i32>表示对Vec<i32>(一个动态数组或向量)引用
fn get_max(num_vec: &[i32]) -> i32 {        // &[i32]表示对整数切片的引用
                                            // &[i32]比&Vec<i32>更通用,因为&[i32]不要求数据必须是Vec<i32>类型的动态数组,
                                            // 它可以是任何整数切片,包括Vec、数组或其他切片
    let mut max = 0;
    if !num_vec.is_empty() {
        for &val in num_vec.iter() { // iter()返回一个迭代器,该迭代器产生对集合中元素的不可变引用。
                                     // 当你使用for &val in num_vec.iter()时,你实际上是在对迭代器
                                     // 产生的每个元素(这些元素本身是对原始数据的引用)进行解引用
        // for &val in num_vec {     // 当你直接在 for 循环中使用 num_vec 时，Rust 通过 IntoIterator trait 
                                     // 为切片 &[i32] 提供了一个迭代器实现。这种情况下，迭代器同样产生对元素的不可变引用。
                                     // 这种方式更简洁，因为你不需要显式调用 .iter() 方法。语法上更直观
            if val > max {
                max = val;
            }
        }
    } else {
        panic!("vector is empty");
    }

    max
}
