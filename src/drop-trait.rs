// 可以为任何类型实现 Drop trait,允许我们在值要离开作用域时执行一些代码

struct CustomSmartPointer {
  data: String,
}

// Drop 被包含在 prelude 中，不用手动引入
impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Drop CustomSmartPointer with data `{}`!", self.data);
  }
}

fn main() {
  let c = CustomSmartPointer { data: String::from("f") };
  let d = CustomSmartPointer { data: String::from("s") };

  // drop 方法不允许直接调用，因为 Rust 仍然会在 main 的结尾对值自动调用 drop，这会导致一个 double free 错误，因为 Rust 会尝试清理相同的值两次。
  // c.drop();

  // 提前调用 std::mem::drop ，释放变量
  drop(d);

  println!("CustomSmartPointers created.");
}
