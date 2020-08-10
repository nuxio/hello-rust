// RefCell 与内部可变性

#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

  *value.borrow_mut() += 10;

  
  println!("a after = {:?}", a);
  println!("b after = {:?}", b);
  println!("c after = {:?}", c);
  println!("value reference count = {}", Rc::strong_count(&value));
  println!("a reference count = {}", Rc::strong_count(&a));
}

pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

// 生命周期参数 a，标识 messenger 必须和 limitTracker 一样长命
impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 0.9 {
      self
        .messenger
        .send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
      self
        .messenger
        .send("Warning: You've used up over 75% of your quota!");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger {
    // sent_messages: Vec<String>,
    sent_messages: RefCell<Vec<String>>, // 使用 RefCell<T> 能够在外部值被认为是不可变的情况下修改内部值
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      self.sent_messages.borrow_mut().push(String::from(message));

      // 下面代码编译不会报错，但是测试会失败
      // let mut one_borrow = self.sent_messages.borrow_mut();
      // let mut two_borrow = self.sent_messages.borrow_mut();
      // one_borrow.push(String::from(message));
      // two_borrow.push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(100);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}
