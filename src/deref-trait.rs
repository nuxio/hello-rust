use std::ops::Deref;

// 指针 是一个包含内存地址的变量的通用概念
// 引用是一种指针，除了引用数据没有任何其他特殊功能，也没有任何额外开销
// 智能指针 smart pointer 是一类数据结构，他们的表现类似指针，但是也拥有额外的元数据和功能
// 普通指针和智能指针的一个额外区别是引用是一类只借用数据的指针；相反，在大部分情况下，智能指针拥有他们指向的数据

// String 和 Vec<T> 其实都是智能指针

// 智能指针通常使用结构体实现。智能指针区别于常规结构体的显著特性在于其实现了 Deref 和 Drop trait

// 最简单直接的智能指针是box，其类型是Box<T>，Box 可以将值存放在堆上而不是栈上，留在栈上的则是指向堆数据的指针，
// 除了数据被存储在堆上而不是栈上之外，box没有性能损失，不过也没有很多额外的功能。
// - 当有一个在编译时未知大小的类型。而又想在需要确切大小的上下文中使用这个类型值的时候
// - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
// - 当希望拥有一个值并只关心它的类型是否实现了特定trait而不是其具体类型的时候

#[derive(Debug)]
enum List {
  Cons(i32, Box<List>), // Box 只提供了间接存储和堆分配，打破了递归引用的问题
  Nil,
}

use crate::List::{Cons, Nil};

// 自定义智能指针
struct MyBox<T>(T); // 一个包含T元素的元祖结构体

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

// 实现 Deref trait 允许我们重载 解引用运算符（dereference operator）*
// 通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待
impl<T> Deref for MyBox<T> {
  type Target = T; // 定义用于此 trait 的关联类型, 关联类型是一个稍有不同的定义泛型参数的方式

  fn deref(&self) -> &T {
    &self.0 // 返回元祖第一个元素
  }
}

fn main() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

  println!("{:?}", list);

  let x = 5;
  let y = &x;
  let yy = Box::new(x);
  let yyy = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y); // 解引用
  assert_eq!(5, *yy); // 像引用一样使用 Box

  assert_eq!(5, *yyy); // 使用自定义智能指针，底层其实是 *(yyy.deref())

  let m = MyBox::new(String::from("Rust"));
  hello(&m); // rust 的解引用强制多态（deref coercions）功能自动帮助我们解引用（待加深理解）
  // hello(&(*m)[..]); // 如果没有解引用强制多态，代码得写成这样才能满足 hello 的参数类型
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}
