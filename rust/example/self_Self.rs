// self和Self
//
// self:
//   self 是一个特殊的关键字，表示当前实例的引用。它通常用于方法的定义中，表示调用该方法的对象。
//   在方法签名中，self 可以有不同的形式：
//      self：表示获取所有权的当前实例。
//      &self：表示对当前实例的不可变引用。
//      &mut self：表示对当前实例的可变引用。

struct MyStruct;

impl MyStruct {
    fn example_method(self) {
        // 这里的 self 是 MyStruct 的一个实例，拥有所有权
    }

    fn example_method_ref(&self) {
        // 这里的 &self 是对 MyStruct 实例的不可变引用
    }

    fn example_method_mut(&mut self) {
        // 这里的 &mut self 是对 MyStruct 实例的可变引用
    }
}

// Self:
//      Self 是一个类型别名，代表实现该方法的类型。它通常用于结构体或枚举的实现块中。
//      Self 可以用来简化代码，避免重复书写类型名，特别是在实现多个方法时。
struct MyStruct;

impl MyStruct {
    fn create() -> Self {  // 表示方法返回一个与调用实例相同类型的值
        // 这里的 Self 表示 MyStruct 类型
        MyStruct
    }

    fn append_bar(self) -> Self {
        // 这里的 Self 也是指 MyStruct 类型
        // 进行一些操作后返回一个 MyStruct 实例
        self // 这里返回当前实例
    }
}

