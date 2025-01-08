fn main() {
    // 创建一个vector的实例
    let mut v: Vec<i32> = Vec::new();  // 创建Vector时，类型标注不是必须的,但这里是必须的,因为这里没有初始值,所以rust
                                   // 不知道这个值的类型,但是如果有初始值,rust就可以推断值的类型
                                   // vector在编译时就必须准确的知道vector中类型的原因在于
                                   // 它需要知道存储每个元素到底需要多少内存
    let v1 = vec![1, 2, 3]; // 此处有初始值,rust可以根据初始值的类型自动做类型推断,所以这里不需要使用类型标注
                            // 使用vec宏来创建一个vector,这个宏会根据我们的初始值创建一个vec

    // 向vec增加元素
    v.push(5);
    v.push(6);
    v.push(7);

    // 访问vecotr的方式
    // 通过索引访问vector的元素
    let third: &i32 = &v[2];   // 返回一个引用
    println!("the index of vector is {}", third);

    // 通过get方式
    match v.get(2) {  // get()方法返回一个Option<T>类型
        Some(third) => println!("the index 3 of vector is {}", third),
        None => println!("vector is none"),
    }

    // rust有两个引用元素的方法的原因是: 程序可以选择如何处理当索引值在Vector中没有对应值的情况
    // 当访问一个索引，但该索引没有对应的值的情况
    // let no_index_value = &v[100];    // 可编译, 但是程序会Panic
    let no_index_value = v.get(100);    // 可编译,并且不会panic,因为get方法返回的是一个Option<T>类型，
                                        // 但访问的索引不存在对应的内容时,get方法会返回一个None,而不会使
                                        // 程序Panic

    // 遍历vector
    for val in &v {
        println!("v value is {}", val);
    }
    
    // 遍历vector,并修改值
    for v in &mut v {
        *v *= 10;    // 先解引用, 再将vector的值乘10
        println!("modify vector: {}", v);
    }

    // 如何用vector存储不同类型的值(vector只能存储相同类型的值)
    // 可以使用枚举类型来作为vector的类型,枚举类型的成员被认为是同一个类型，即那个枚举的类型
    #[derive(Debug)]
    enum School {
        Student(String),
        Teacher {id: i32, name: String},
    }

    let mut school = vec![
        School::Student(String::from("david")),  // 枚举类型的成员赋值
        School::Teacher {id: 9999, name: String::from("teacher")},
    ];

    for v in &school {
        println!("student name: {:#?}", v);
    }
}
