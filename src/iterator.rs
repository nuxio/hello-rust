// rust 中，迭代器是惰性的，在调用方法使用迭代器之前它都不会有效果

// 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait，看起来像这样：
// pub trait Iterator {
//   type Item; // trait 的关联类型

//   fn next(&mut self) -> Option<Self::Item>; // 消费适配器（consuming adaptors）
// }

#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filter_by_size() {
  let shoes = vec![
    Shoe {
      size: 10,
      style: String::from("sneaker"),
    },
    Shoe {
      size: 13,
      style: String::from("sandal"),
    },
    Shoe {
      size: 10,
      style: String::from("boot"),
    },
  ];

  let in_my_size = shoes_in_my_size(shoes, 10);

  assert_eq!(
    in_my_size,
    vec![
      Shoe {
        size: 10,
        style: String::from("sneaker"),
      },
      Shoe {
        size: 10,
        style: String::from("boot"),
      },
    ]
  )
}

struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

// 自定义 iterator
impl Iterator for Counter {
  type Item = u32;

  // next 是 iterator 必须要实现的方法
  // 也是其他方法的基础，实现了 next 方法后，就可以调用 zip filter 等方法
  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;
    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

#[test]
fn calling_next_directly() {
  let mut counter = Counter::new();

  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), Some(4));
  assert_eq!(counter.next(), Some(5));
  assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
  let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();

  assert_eq!(18, sum);
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

  // collect 方法消费了迭代器，将迭代适配器返回的值收集进一个 vector 并返回
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

  assert_eq!(v2, vec![2, 3, 4]);
}

// 迭代器，作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能的代码
// 迭代器是Rust的 零成本抽象 zero-cost abstractions 之一，意味着不会引入运行时开销
