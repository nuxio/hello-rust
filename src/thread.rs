// 线程

use std::thread;
use std::time::Duration;

fn main() {
  // 开一个新线程。主线程结束后，新建的线程也会被销毁
  thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1)); // sleep 方法调用强制线程停止执行一段时间，这会允许其他不同的线程运行
    }
  });

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  // JoinHandle
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the second spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  // join 会阻塞当前线程直到 handle 所代表的线程结束
  handle.join().unwrap();

  // move 闭包
  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || { // move 会强制获取闭包所使用的值的所有权
    println!("Here's a vector: {:?}", v);
  });

  handle.join().unwrap();

  println!("MAIN thread over.");
}
