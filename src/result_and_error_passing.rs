use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

// panic! 宏代表一个程序无法处理的状态，并停止执行而不是使用无效或不正确的值继续处理
// Result 枚举代表操作可能会在一种可以恢复的情况下失败

fn main() {
  // let file = File::open("hello.txt"); // 返回 Result<T, E> 类型的值

  // 使用 match 处理不同的情况
  // let file = match file {
  //   Ok(file) => file,
  //   Err(error) => match error.kind() {
  //     ErrorKind::NotFound => match File::create("hello.txt") {
  //       Ok(fc) => fc,
  //       Err(e) => panic!("Failed to create file: {:?}", e),
  //     },
  //     other_error => panic!("Failed to open file: {:?}", other_error),
  //   },
  // };

  // 由于 match 写起来比较长，可以使用 Result 类型的 unwrap 方法
  // 如果 open 返回的 Result 值是 Ok，unwrap 会返回 Ok 中的值，如是 Err，unwrap 会自动调用 panic!
  // let file = File::open("world.txt").unwrap();

  // 使用 expect 可以设置错误信息
  // let f = File::open("world.txt").expect("Failed to open world.txt");

  // let name = read_username_from_file().unwrap();
  let name = read_username_from_file_exx_simple().unwrap();

  println!("name is: {}", name);
}

// 传播错误，返回错误对象
fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e), // 函数到此结束，返回 Err 包裹的错误信息
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  } // 这里没有分号，返回 match 表达式的结果
}

// 传播错误的简写
fn read_username_from_file_simple() -> Result<String, io::Error> {
  // let f = File::open("hello.txt");
  // let mut f = match f {
  //   Ok(file) => file,
  //   Err(e) => return Err(e), // 函数到此结束，返回 Err 包裹的错误信息
  // };
  //
  // Result 后的 ? 运算符和 match 表达式一致，下面这句和上面这坨效果一致
  let mut f = File::open("hello.txt")?; // 、? 运算符和match有一点不同
                                        // 在返回错误时，? 运算符会将错误传递给 from 函数
                                        // from 函数会把错误转成当前函数返回类型所指定的错误类型
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s) // 隐式返回 Ok(s)
}

fn read_username_from_file_ex_simple() -> Result<String, io::Error> {
  let mut s = String::new();

  // 链式调用
  File::open("hello.txt")?.read_to_string(&mut s)?;

  Ok(s)
}

// 直接调用方法
fn read_username_from_file_exx_simple() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
