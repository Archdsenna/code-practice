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
        print!("\n\nInput you choose: ");
        io::stdout().flush().unwrap();
        // unwrap()的作用:
        //  在 Rust 中，unwrap() 方法是一个非常常用的方法，它用于处理 Result 或 Option 类型的值。
        //  这两种类型通常用于错误处理或表示可能缺失的值。
        //
        // unwrap() 方法的作用
        //      (1) 当用于 Result<T, E> 类型时，如果 Result 是 Ok(T)，unwrap() 会返回 T，即成功的值；
        //          如果是 Err(E)，则会导致程序 panic，并显示错误信息。
        //      (2) 当用于 Option<T> 类型时，如果 Option 是 Some(T)，unwrap() 会返回 T；如果是 None，
        //          则同样会导致程序 panic，并显示一条默认的错误信息。
        // 总结:
        //      如果Result的值是成员Ok,unwrap()会返回Ok中的值。如果Result的值是成员Err,unwrap会为我们调用panic
        //
        // 使用 unwrap() 的目的是为了简化错误处理：如果 flush() 成功，程序将继续执行；
        // 如果失败，程序将会 panic，并停止执行，同时提供错误信息。
        //
        // 不使用 unwrap() 的影响
        //      不使用 unwrap() 并不意味着问题，而是需要你采取其他方式来处理可能的错误。
        //      不使用 unwrap()，你可以选择更安全的错误处理方式，例如使用 expect() 提供
        //      更具体的错误信息，或者使用 match 语句来分别处理成功和错误的情况，这样可
        //      以避免程序因为错误而突然终止
        //
        // 总结
        //      unwrap() 是一个快速处理错误的方法，但它会在遇到错误时导致程序 panic。
        //      在生产环境中，更推荐使用显式的错误处理方式，以提供更稳定和可控的程序行为。

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
        part_staff.staffs.extend(old_staff.iter().cloned());  // extend方法可以合并两个vector
    }

    // HashMap的instert方法,如果插入的键已经存在于HashMap中,那么该方法会替换掉原有的值,
    // 返回一个Option<T>类型的值,其中包含被替换的旧值
    company.insert(part_staff.part_name.clone(), part_staff.staffs.clone());  // clone()是一个方法,直接
                                                                                    // 作用于单个对象,用于创建
                                                                                    // 该对象的一个副本
                                                                                    // cloned()是一个迭代器适配器，用于将
                                                                                    // 迭代器中的每个元素进行克隆操作，返回
                                                                                    // 一个新的迭代器,其元素是原始元素的副本
                                                                                    //
                                                                                    // 如何选择用哪个？
                                                                                    // 取决于你的具体需求,如果需要复制单个对象，
                                                                                    // 使用clone();如果需要在迭代过程中复制元素,
                                                                                    // 使用cloned()
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

