mod mod_1;

use crate::mod_1::fn_1;

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn main() {
  let x = value_in_cents(Coin::Quarter(UsState::Alabama));

  println!("{}", x);

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  let some_u8_value = 0u8;
  match some_u8_value {
    // match 相当于 switch 语法
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // _ 相当于 default
  }

  // 0u8 意思是 0 作为 u8 也就是8位无符号整数，也就是 0
  let some_u8_value = Some(0u8);

  if some_u8_value == Some(3) {
    println!("if three");
  }

  match some_u8_value {
    Some(3) => println!("match three"),
    _ => (),
  }

  if let Some(0) = some_u8_value {
    println!("if let zero");
  }

  let q = Coin::Quarter(UsState::Alaska);

  // if let pattern = expression
  // if let 是 match 的语法糖，这个例子与下面的 value_in_cents 一致
  // value 将会绑定为 q 的值
  if let Coin::Quarter(value) = q {
    println!("q is {:?}", value);
  }


  fn_1::bb();
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}
