// “不要通过共享内存来通讯；而是通过通讯来共享内存。”（“Do not communicate by sharing memory; instead, share memory by communicating.”）

// mpsc 多个生产者，单个消费者（multiple producer, single consumer）
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  // tx 发送者 rx 接收者
  // let (tx, rx) = mpsc::channel();

  // thread::spawn(move || {
  //   let val = String::from("hi");

  //   tx.send(val).unwrap(); // 如果接收端已经被废弃了，将没有发送值的目标，所以发送操作可能会返回错误
  // });

  // let received = rx.recv().unwrap();

  // println!("Got: {}", received);

  // ---------- part 2
  // let (tx, rx) = mpsc::channel();

  // thread::spawn(move || {
  //   let vals = vec![
  //     String::from("hi"),
  //     String::from("from"),
  //     String::from("the"),
  //     String::from("thread"),
  //   ];

  //   for val in vals {
  //     tx.send(val).unwrap();
  //     thread::sleep(Duration::from_secs(1));
  //   }
  // });

  // for received in rx {
  //   println!("Got: {}", received);
  // }

  // ---------- part 3
  let (tx, rx) = mpsc::channel();

  // 克隆发送端
  let tx1 = mpsc::Sender::clone(&tx);

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  // 每次执行输出都不一样
  for received in rx {
    println!("Got: {}", received);
  }
}
