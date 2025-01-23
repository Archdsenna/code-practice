// 泛型函数：具有泛型类型参数的函数
//
// Q: 什么是泛型类型参数?
// A: 泛型类型函数，具有类型参数，这个类型就是泛型类型，泛型类型使用<>括号括起来
//    类型参数的声明位于函数名称与参数列表中间的尖括号<>中,如下:
//      fn foo<T>(var: &[T]) -> T {}   // T为类型参数的名字
//    可以这样理解这个定义,函数foo有泛型类型T,它有个参数var,其类型是元素为T的slice

use std::cmp;

// 泛型函数
fn find_max<T>(list: &[T]) -> T {  // 泛型函数, <>中的是泛型参数的名称
    if !list.is_empty() {
        let mut max = list[0];

        for &val in list {
            if val > max {
                max = val;
            }
        }

        max
    } else {
        panic!("list is empty");
    }
}

fn main() {
    let char_list = vec!['v', 'e', 'r', 'y'];
    let num_list = vec![99, 89, 38, 66, 100, 21];

    println!("max char: {}", find_max(&char_list));
    println!("max number: {}", find_max(&num_list));

    // 泛型参数类型的结构体
    let integer = Point { x: 1, y: 9 };
    let float = Point { x: 1.0, y: 8.0  };
    // let p = Point { x: 5, y: 9.0  };    // 编译器会报错,当把5赋值给x时,就告诉了编译器这个Point<T>的实例中的泛型T是整型的
    let p = PointA { x: 5, y: 9.0  };    
    println!("{}", p.x());
}


// 结构体定义中的泛型
// 同样可以用<>语法来定义结构体,它包含一个或多个泛型参数类型字段
struct Point<T> {  // <>中的是泛型参数的名称, 结构体只使用了一个泛型类型
    // x和y是相同的类型
    x: T,
    y: T,
}

// 具有两个泛型参数的结构体
struct PointA<T, U> {
    x: T,
    y: U,
}


// 方法定义中的泛型
// 在为结构体和枚举实现方法时, 一样可以使用泛型
impl<T> PointA<T> {
    fn x(&self) -> &T {
        self.x
    }
}
