// 线程间共享状态

// Arc 原子引用计数类型 atomically reference counted
// Mutex 互斥器，mutual exclusion
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
  // 创建一个互斥器，包含要在线程之间共享的数据
  // 使用 Arc 包装互斥器，让互斥器可以有多个所有者，才能在多线程访问 // Rc 在多线程之间共享不安全
  let counter = Arc::new(Mutex::new(0));
  let mut handlers = vec![];

  for _ in 0..10 {
    // 引用 +1
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      // 使用lock方法获取锁，以访问互斥器中的数据。如果另一个线程拥有锁，并且那个线程panic了，则lock调用会失败。
      let mut num = counter.lock().unwrap();

      *num += 1;
    });

    handlers.push(handle);
  }

  // 等待所有线程执行完成
  for handle in handlers {
    handle.join().unwrap();   
  }

  println!("Result: {}", *counter.lock().unwrap());
}
