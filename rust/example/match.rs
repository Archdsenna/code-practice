#[derive(Debug)]
enum State {
    A,
    B,
    C,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {  // 如果调用value_in_cents(Coin::Quarter(State::c)), 
                  // coin的值将是Coin::Quarter(State::c),
                  // 当将值与每个分支相比较时,没有分支会匹配,直到遇到Coin::Quarter(state),
                  // 这时,state绑定的将会是值State::c
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state is {:?}", state);
            25
        },
    }
}

fn push_one(o: Option<i32>) -> Option<i32> {
    match o {
        Some(o) => Some(o + 1),
        None => None,
    }
}

fn main() {
    let x = push_one(Some(5));
    println!("x is {:?}", x);

    let none = push_one(None);
    println!("none is {:?}", none);

    // match匹配的通用模式
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),  // 定义一个变量other,匹配所有其他情况
        // _ => do_nothing(),   // _表示通配模式,可以匹配任意值而不绑定到该值 
        _ => (),  // 可以使用单元值(在元组类型中提到的空元组()),表示什么也不做,即空操作
                  // 在这里,我们明确告诉rust我们不会与前面模式不匹配的值,并且这种情况下我们不想运行任何代码
    }


    // if let 语句: 处理只匹配一个模式的值,而忽略其他模式的情况
    let some_u8_val = Some(0u8);
    match some_u8_val { // 和上面是同一个变量
        Some(3) => println!("three"),  // 匹配一个Option<T>的值,并且仅当值为3时执行代码
        _ => (),
    }

    // 下面的if let语句与上面match逻辑相同
    if let Some(3) = some_u8_val {
        println!("three");
    }

    // if let 语句
    if let Some(9) = some_u8_val {
        println!("nine");
    } else {
        println!("not nine");
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(r: u8) {}
fn do_nothing() {}
