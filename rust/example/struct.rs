// 定义一个结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}  // 结构体定义的结尾没有;号

// 元组结构体: 元组结构体具有命名的类型,但没有命名字段的结构体,其字段通过位置进行访问
struct Color(i32, i32, i32); // 元组结构体的定义结尾有;号
struct Point(i32, i32, i32);

// 可以在函数体的最后一个表达式中构造一个结构体的新实例,来隐式的返回这个实例
fn build_user(username: String, email: String) -> User {
    User {  // 隐式返回新实例
        // 如下两个字段可以使用 字段初始化简写语法进行简化
        // username: name,
        // email: email,
        // 简化如下：
        username,       // 参数名与字段名完全相同的情况下,可以直接写参数名
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // 创建结构体的实例
    // 注意整个实例必须是可变的, rust并不允许只将某个字段标记为可变
    let mut user1 = User {   // user1的字段可变
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 获取结构体某个成员的值可以使用.号
    println!("email: {}", user1.email);

    // 改变结构体成员的值
    user1.username = String::from("david");
    println!("email: {}", user1.username);

    let user2_name = String::from("panda");
    let user2_email = String::from("sichuan.com");
    let user2 = build_user(user2_name, user2_email);
    println!("user2 name: {}", user2.username);

    // 使用结构体更新语法从其他实例创建新实例
    let user3 = User {
        email: String::from("apple"),
        // 结构体更新语法,节省代码
        ..user1    // ..语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值
                   // 可以选择以任何顺序为任意字段指定值,而不用考虑结构体定义中字段的顺序
                   // 结构体更新语法就像带有=的赋值,因为它移动了数据
                   // 因此,之后不能再使用user1,因为user1的所有权发生了转移,user1不要拥有数据的所有权
                   // 如果我们给user3的username和email字段赋予新的String值,只使用active和sign_in_count值,
                   // 那么user1在创建user3之后仍然有效,因为active和sign_in_count的类型是实现了Copy trait的类型
                   //
                   // 补充: Rust有一个叫做Copy trait的特殊标注,可以用在整型这样的可以存储在栈上的类型上。
                   //       如果一个类型实现了Copy trait, 那么一个旧的变量在将其值赋值给其他变量后仍然可用,
                   //       rust不允许自身或其任何部分实现了drop trait的类型使用Copy trait。
                   //       如下是一些Copy的类型:
                   //           所有整型, 比如u32
                   //           布尔类型
                   //           所有浮点类型,比如f64
                   //           字符类型,char
                   //           元组, 当且仅当其包含的类型也都实现了Copy的时候,比如(i32, i32)实现了Copy，但(i32, String)就没有
    };
    println!("user3 name is {}", user3.username);

    // 元组结构体
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);  // 注,black和origin值的类型不同,因为它们是不同的元组结构体的实例
                                  // 元组结构体成员的的访问使用.
    println!("black0 is {}", black.0);   // 元组结构体通过位置进行访问
}
