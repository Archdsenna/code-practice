// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
// 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，
// 或者公司每个部门的所有员工按照字典序排列的列表。

use std::io::{self, Write};
use std::collections::HashMap;

struct Partment {
    part_name: String,
    staffs: Vec<String>,
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();  // 堆上分配内存
    let mut text_str = String::new();                        // 堆上分配内存

    loop {
        println!("Please input text to add staff into part, eg: A(a)dd David to Sales:");
        println!("======================================");

        text_str.clear();
        io::stdin().read_line(&mut text_str).expect("input error");
        // println!("{}", text_str);

        let mut part_staff = extract_staff_part_from_text(&mut text_str);
        add_new_into_company(&mut part_staff, &mut company);
        
        // println!("{:#?}", company);

        print!("\nNow you can get all staffs of partment which you choose, please choose one as following: \n");
        for part in company.keys() {
            print!("{} ", part);
        }
        // io::stdout().flush().unwrap();
        print!("\n\nInput you choose: ");
        io::stdout().flush().unwrap();

        let mut p = String::new();
        io::stdin().read_line(&mut p).unwrap();
        get_staffs_by_part(&mut p , &company);

        println!("\nAll department employees are sorted in lexicographical order");
        println!("--------------------------------------------------------------");
        all_part_of_staffs_by_lex_sort(&mut company);

        println!("\nWould you like to add staff next? Please input 'n' to exit or other to continue");
        let mut choose = String::new();
        io::stdin().read_line(&mut choose).unwrap();

        if choose.trim().eq("n") {
            break;
        }
    }

    println!("{:#?}", company);
    
}

fn all_part_of_staffs_by_lex_sort(company: &mut HashMap<String, Vec<String>>) {
    for (_, staff_vec) in company.iter_mut() {
        staff_vec.sort();
    }
    println!("{:#?}", company);
}

fn add_new_into_company(part_staff: &mut Partment, company: &mut HashMap<String, Vec<String>>) {
    if let Some(old_staff) = company.get(&part_staff.part_name) {
        part_staff.staffs.extend(old_staff.iter().cloned());
    }

    // HashMap的instert方法,如果插入的键已经存在于HashMap中,那么该方法会替换掉原有的值,
    // 返回一个Option<T>类型的值,其中包含被替换的旧值
    company.insert(part_staff.part_name.clone(), part_staff.staffs.clone());  
}

fn get_staffs_by_part(part_name: &str, company: &HashMap<String, Vec<String>>) {
    let staff_vec = company.get(part_name.trim()).unwrap();

    print!("\nAll staffs of {} : ", part_name.trim());

    for staff in staff_vec.iter() {
        print!("{} ", staff);       
    }
    println!();
}

// 文本接口, 向公司的部门中增加员工名字
fn extract_staff_part_from_text(text_str: &mut str) -> Partment {
    let action = vec!["add", "Add", "to"];
    let words = text_str.trim().split_whitespace();
    let mut staff_vec:  Vec<String> = Vec::new();
    let mut part = String::new();

    for word in words {
        if !action.contains(&word) {
            staff_vec.push(word.to_string());
        }
    }

    part.push_str(staff_vec.pop().unwrap().as_str());
    Partment {
        part_name: part,
        staffs: staff_vec,
    }
}

