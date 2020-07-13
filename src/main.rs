#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 {
      panic!(
        "Guess value must be greater than or equal to 1, got {}.",
        value
      );
    } else if value > 100 {
      panic!(
        "Guess value must be less than or equal to 100, got {}.",
        value
      );
    }

    Guess { value }
  }
}

// 当使用 `cargo test` 命令运行测试时，Rust 会构建一个测试执行程序用来调用标记了 test 属性的函数
#[cfg(test)]
mod tests {
  use super::*; // 在模块 tests 内引入 Rectangle

  #[test] // 这个属性表示这是一个测试函数
  fn it_works() {
    // 断言是否相等，底层使用了 ==
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn another_works() {
    // 断言是否不等，底层使用了 !=
    assert_ne!(2 + 2, 5);
  }

  #[test]
  fn custom_fail_message() {
    assert!(false, "This is the custom fail message, {}", "Yeah");
  }

  #[test]
  #[ignore] // 忽略此测试
  fn another() {
    panic!("Make mistake!");
  }

  #[test]
  fn lager_can_hold_smaller() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    // 断言，只判断 true 和 false
    assert!(larger.can_hold(&smaller));
  }

  #[test]
  #[should_panic(expected = "Guess value must be less than or equal to 100")]
  fn greater_than_100() {
    Guess::new(200); // 捕获 panic!
  }

  #[test]
  fn another_it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }
}

pub fn add_two(a: i32) -> i32 {
  internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
  a + b
}

fn main() {}
