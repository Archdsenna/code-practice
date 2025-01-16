// rust的包和crate

// crate是一个二进制项或者库
// cargo root是一个源文件,Rust编译器以它为起始点,并构成你的crate的根模块
// 包是提供一系列功能的一个或多个crate
// 一个包会包含一个Cargo.toml, 阐述如何去构建这些crate

// Cargo遵循一个约定
//  1. src/main.rs就是一个与包同名的二进制crate的crate根
//  2. 同样的, Cargo知道如果包目录中包含src/lib.rs, 则包带有与其同名的库crate,且src/lib.rs是crate根
// src/main.rs和src/lib.rs被称为crate根, 如此称呼的原因是,这两个文件中任意一个的内容会构成名为crate的模块,
// 且该模块位于crate的被称为模块树的模块结构的根部
//
//  也就是说, crate有两种,分别是二进制crate和库crate, src/main.rs是与包同名的二进制crate的根,
//  src/lib.rs是与包同名的库crate的根

//  *crate根文件将由Cargo传递给rustc来实际构建库或者二进制项目

// 通常将文件放在src/bin目录下,一个包可以拥有多个二进制crate,每个src/bin下的文件都会被编译成一个独立的二进制crate
