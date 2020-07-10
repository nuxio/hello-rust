use std::fmt::Display;

// trait 相当于其它语言的 interface
pub trait Summary {
  fn summary(&self) -> String;
}

// 带默认实现的 trait
pub trait ReadMore {
  fn author(&self) -> String;

  fn read_more(&self) -> String {
    format!("(Read more from {}...)", self.author()) // 默认实现可以调用相同 trait 内的其他方法
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

// 为 NewsArticle 实现 Summary 特性
// 只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现trait
// 此限制被称为 相干性 coherence 或者 孤儿规则 orphan rule
// 这条规则确保了其他人编写的代码不会破坏你的代码
impl Summary for NewsArticle {
  fn summary(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

// 使用默认实现
impl ReadMore for NewsArticle {
  fn author(&self) -> String {
    format!("@{}", self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summary(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summary());

  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
    ),
  };

  println!("New article available! {}", article.read_more());

  notify(tweet);
  notify(article);

  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {}", result); // y
}

// trait 作为参数，相当于是指定类型？
pub fn notify(item: impl Summary) {
  println!("Breaking news! {}", item.summary());
}

// trait bound，可以将 trait 赋值到泛型
pub fn notify2<T: Summary>(item1: T, item2: T) {
  println!("Breaking news 1! {}", item1.summary());
  println!("Breaking news 2! {}", item2.summary());
}

// 通过 + 操作符指定多个 trait bound
pub fn notify3<T: Summary + Display>(item: T) {
  println!("Breaking news ! {}", item.summary());
}

pub trait Clone {
  fn clone(&self) -> bool;
}

pub trait Debug {
  fn debug(&self) -> String;
}

// 如果多个参数，都有 trait bound，会比较长，杂乱
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
  2
}

// 使用 where 语句来收拢 trait bound
fn some_function_where<T, U>(t: T, u: U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
{
  2
}

// 返回值也可以实现 trait
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}

// 大于运算符 > 比较两个 T 类型的值
// 这个运算符被定义为标准库中 trait std::cmp::PartialOrd 的一个默认方法
// PartialOrd 位于 prelude，直接添加 trait bound
// 又由于使用了泛型，没有 Copy trait，将数组中的值复制到变量中会编译报错，所以还需要加上 Copy trait bound
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

// 使用 trait bound 有条件的实现方法

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    // 注意这里 Self 是大写开头
    Self { x, y }
  }
}

impl<T: Display + PartialOrd> Pair<T> { // 只有实现了 Display 和 PartialOrd trait 后才会有 cmp_display 方法
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

// trait 和 trait bound 让我们使用泛型类型参数来减少重复，并仍然能够向编译器明确指定泛型类型需要拥有哪些行为
