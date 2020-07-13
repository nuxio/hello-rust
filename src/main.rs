// rust 中，迭代器是惰性的，在调用方法使用迭代器之前它都不会有效果

// 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait，看起来像这样：
pub trait Iterator {
  type Item; // trait 的关联类型

  fn next(&mut self) -> Option<Self::Item>; // 消费适配器（consuming adaptors）
}

fn main() {
  let v1 = vec![1, 2, 3];

  // into_iter 返回拥有所有权的迭代器
  // iter_mut 返回可变引用
  let v1_iter = v1.iter(); // 返回不可变引用

  let total: i32 = v1_iter.sum(); // sum 会获取迭代器所有权并且反复调用 next 方法，所以也是一个消费适配器

  // Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），他们允许我们将当前迭代器变为不同类型的迭代器。
  let v1: Vec<i32> = vec![1, 2, 3];

  // 调用迭代器适配器 map 来创建一个新迭代器，由于迭代器是惰性的
  // 下面这句代码相当于还停留在声明阶段
  // v1.iter().map(|x| x + 1);

  // collect 方法消费了迭代器
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

  assert_eq!(v2, vec![2, 3, 4]);
}
