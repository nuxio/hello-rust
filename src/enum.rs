enum IpAddrKind {
  V4,
  V6,
}

struct IpAddr {
  Kind: IpAddrKind,
  address: String,
}

// 更方便的定义
enum AnotherIpAddr {
  V4(u8, u8, u8, u8), // address 可以有不同的类型
  V6(String),
}

enum Message {
  // enum 内可以嵌套各种类型的数据
  Quit,
  Move { x: i32, y: i32 }, // 匿名 struct
  Write(String),
  ChangeColor(i32, i32, i32),
}

// rust 内置了一个 Option 枚举类型，可以直接引用
// enum Option<T> {
//   Some(T),
//   None,
// }

fn main() {
  let home = IpAddr {
    Kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    Kind: IpAddrKind::V6,
    address: String::from("::1"),
  };

  let home1 = AnotherIpAddr::V4(127, 0, 0, 1);

  let loopback1 = AnotherIpAddr::V6(String::from("::1"));

  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;
}
