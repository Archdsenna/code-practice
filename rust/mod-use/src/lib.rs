mod network;   // 使用mod.rs的方式组织模块

mod school;   // 不使用mod.rs的方式组织模块

fn func() {
    // 引用network模块的内容
    network::server::server_func();
    network::client::client_func();

    // 引用school模块的内容
    school::student::student_say_hello();
    school::teacher::teacher_say_hi();
}

