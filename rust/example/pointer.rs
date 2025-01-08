fn main() {
    let x = 5;
    let r = &x as *const i32;

    let mut y = 10;
    // let m = &mut y as *mut i32;
    let m = &y as *const i32;

    // 要通过裸指针访问或修改数据必须在unsafe块中进行
    unsafe {
        println!("r pointer to : {}", *r);
        println!("m pointer to : {}", *m);
        // *m *= 10;
        // println!("m pointer to : {}", *m);
    }
}
