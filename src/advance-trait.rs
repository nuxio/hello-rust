// 高级 trait

// 关联类型在trait定义中指定占位符类型
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
  x: u32,
}

// 实现 trait 时指定 Item 类型
impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    // --snip--
    Some(self.x)
  }
}

// 运算符重载 和 默认泛型类型参数
// + 加号运算符的 trait 定义大概如下
// trait Add<RHS=Self> { // RHS 是 right hand side, RHS 的类型将是默认的 Self 类型
//   type Output;

//   fn add(self, rhs: RHS) -> Self::Output;
// }

use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

// 重载 Millimeters + Meters 这个加号的操作
impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + other.0 * 1000)
  }
}

// 完全限定语法与消歧义，调用相同名称的方法
// <Type as Trait>::function(receiver_if_method, next_arg, ...);
trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("puppy")
  }
}

// 在 trait 的默认实现中调用另外一个 trait 的方法
// 那么在实现这个trait的结构体中还必须实现另外一个trait的方法
use std::fmt;

trait OutlinePrint: fmt::Display {
  fn outline_print(&self) {
    let output = self.to_string(); // to_string 方法来自于 Display trait
    let len = output.len();

    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

struct Point {
  x: i32,
  y: i32,
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl OutlinePrint for Point {}

// newtype 模式用于在外部类型上实现外部trait
// 比如在 Vec<T> 上实现 Display ，由于都是外部的，可以包裹一层绕开 孤儿规则 的限制
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
  let x = Millimeters(50);
  let y = Meters(1);

  let z = x + y;

  println!("z is {:?}", z);

  println!("A baby dog is called a {}", Dog::baby_name());
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // 使用不同trait的相同名称的方法

  let p = Point { x: 0, y: 8 };

  p.outline_print();

  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}
