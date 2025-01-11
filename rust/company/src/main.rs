// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
// 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，
// 或者公司每个部门的所有员工按照字典序排列的列表。

use std::io;
use std::collections::HashMap;

fn main() {
    let mut company: HashMap<&str, Vec<&str>> = HashMap::new();
    
    // let mut staffs = vec!["Amir"];
    // company.insert("Sales", staffs);

    println!("Please input text to add staff into part, eg: A(a)dd David to Sales:");

    let mut text_str = String::new();
    io::stdin().read_line(&mut text_str).expect("input error");
    let staff_part = extract_staff_part(&mut text_str);
    
    company.insert(staff_part.0.as_str(), staff_part.1);
    println!("{:#?}", company);

    
}

// fn process_text

// 文本接口, 向公司的部门中增加员工名字
fn extract_staff_part(text_str: &mut str) -> (String, Vec<&str>) {
    let action = vec!["add", "Add", "to"];
    let words = text_str.trim().split_whitespace();
    let mut staff_vec = Vec::new();
    let mut part = String::new();

    for word in words {
        if !action.contains(&word) {
            staff_vec.push(word);
        }
    }

    part.push_str(staff_vec.pop().unwrap());
    (part, staff_vec)
}

