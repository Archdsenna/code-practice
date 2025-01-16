// 返回值与作用域
fn main() {
    let s1 = gives_ownership();   // gives_ownership将返回值移给s1

    let s2 = String::from("hello");  // s2进入作用域

    let s3 = takes_and_gives_back(s2);  // s2被移动到takes_and_gives_back中,
                                        // 它也将返回值移给s3
} // 这里,s3移出作用域并被丢弃。s2也移出作用域,但已被移走,所以什么也不发生。
  // s1移出作用域并被丢弃
  // 当持有堆中数据值的变量离开作用域时,其值将会通过`drop`被清理掉,
  // 除非数据被移动为另一个变量所有

fn gives_ownership() -> String { // gives_ownership将返回值移动给调用它的函数
    let some_string = String::from("yours"); // some_string进入作用域

    some_string     // 返回some_string并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String { // a_string进入作用域
    a_string  // 返回a_string，并移出给调用的函数
} 
