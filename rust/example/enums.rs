// rust中的枚举

enum IpAddrKind<'a> {
    // rust中,任何引用都需要一个明确的生命周期,下面的写法错误
    // V4(&str),     // 称为枚举的成员
    // V6(&str),
    // 正确写法如下:
    V4(&'a str),
    V6(&'a str),
}

impl<'a> IpAddrKind<'a> {
    fn from(ip_addr: &str) -> IpAddrKind {  // 必须确保字符串的生命周期覆盖实例的生命周期,即当调用from时,
                                            // 必须确保此时字符串的生命周期是有效的,如果此时&str已经无效,
                                            // 那么from函数将访问一个悬垂引用,所以如果一个函数的参数是引用类型,
                                            // 那么必须使用生命周期标注,显式确保该&str类型在调用from时依然有效
        if ip_addr.contains(".") {
            IpAddrKind::V4(ip_addr)  // 注意这里不能加分号,因为要返回值,所以这里是表达式
        } else {
            IpAddrKind::V6(ip_addr)
        }
    }

    fn get_ip(&self) ->&str {
        // match语句会自动解引用, rust的模式匹配解引用特性
        match self {   // Q: 为什么不写成 match &self?
                       // A: 当使用match语句匹配一个引用时,rust会自动进行所谓的"解引用强制",
                       //    这意味着可以直接匹配引用的值,而不需要显式的解引用,
                       //    rust的模式匹配会自动解引用多层引用,写成match &eslf也是可以的,但是没有必要
            IpAddrKind::V4(ipv4) => ipv4, // 每个匹配分支可以直接访问实例的数据,而不是引用
            IpAddrKind::V6(ipv6) => ipv6, // 但匹配到一个符合的模式时,值会进入相关联的代码块,并在执行中被使用
        }
    }
}

fn main() {
    // 创建2个IpAddrKind枚举类型的实例
    // let v4 = IpAddrKind::V4;   // v4和v6都是IpAddrKind类型的
    // let v6 = IpAddrKind::V6;

    let v4 = IpAddrKind::from("192.168.0.1");

    println!("ipv4 is {}", v4.get_ip());

}

// fn route(ip_type: IpAddrKind) {   // 参数为IpAddrKind类型
// }

enum IpKind {
    v4,
    v6,
}

struct Ip {
    kind: IpKind,
    addr: String,
}

let home = Ip {
    kind: IpKind::v4,
    addr: String::from("172.0.0.1"),
};

let loopback = Ip {
    kind: IpKind::v6,
    addr: String::from("::1");
};

// enum类型IpKindStr的成员都关联了String值
enum IpKindStr {
    v4(String),
    v6(String),
}
let ip4 = IpKindStr::v4(String::from("172.0.0.1"));
let ip6 = IpKindStr::v6(String::from("::1"));


// tuple方式
enum IpKindTuple {
    v4(u8, u8, u8, u8),
    v6(String),
}

let i4 = IpKindTuple::v4(127, 0, 0, 1);
