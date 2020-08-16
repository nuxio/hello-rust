// 类型别名 Kilometers 只是一个别名，代表 i32
type Kilometers = i32;

// 可组合
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
  // --snip--
}

fn returns_long_type() -> Thunk {
  // --snip--
  let f: Thunk = Box::new(|| println!("hi"));
  f
}

// 可带泛型
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
  fn write(&mut self, buf: &[u8]) -> Result<usize>;
  fn flush(&mut self) -> Result<()>;

  fn write_all(&mut self, buf: &[u8]) -> Result<()>;
  fn write_fmt(&mut self) -> Result<()>;
}

// never type 从不返回值
fn bar() -> ! {
  // 永无止境的循环是 never type
  // loop {
  //   print!("forever and ever.");
  // }
  // continue 是 never type
  panic!("panic! 是 never type")
}

// 动态大小类型和 Sized trait
fn generic<T: ?Sized>(t: &T) {
  // --snip--
}

fn main() {
  let x: i32 = 5;
  let y: Kilometers = 5;

  println!("x + y = {}", x + y);
}
