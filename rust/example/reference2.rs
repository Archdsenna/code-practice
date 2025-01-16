fn main() {
    let x = 2;
    let y = &x;

    // 打印引用y指向的内容
    // println!("{}", y);
    // 打印引用y保存的地址
    println!("{:p}", y);

    // 打印引用y自身的地址
    let y_addr = &y as *const _ as usize;
    println!("Address of y: {:x}", y_addr);
    // let y_addr = &y;
    // println!("The address of y reference is {:x}", &y);
    //  y = &x
    //  z = &y = &&x
    //     解引用    解引用
    //  &y  --->  y   --->  x
    // (&&x)     (&x)      (x)
    // println!("Address of y: {:x}", &y);
    // println!("Address of y: {:x}", &(&x));
    // println!("Address of y: {:x}", &&x);
    // println!("Address of y: {:x}", &x);
    // println!("Address of y: {:x}", x);
    let z = &y;
    println!("{:p}", z);
}
