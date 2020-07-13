use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
  T: Fn(u32) -> u32, // 因为结构体需要定义每个字段的类型，闭包要存放在结构体中也需要定义类型
                     // Fn 是一个 trait
{
  calculation: T,
  value: HashMap<u32, u32>, // 用于缓存计算过的数值
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: HashMap::new(),
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    let value = self.value.get(&arg);

    match value {
      Some(v) => *v,
      None => {
        let a = (self.calculation)(arg);
        self.value.insert(arg, a);
        a
      }
    }
  }
}

// 闭包是可以保存进变量或者作为参数传递给其它函数的匿名函数
fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_closure = Cacher::new(|num| {
    // 闭包声明，未指定参数类型的情况下，编译器会根据调用闭包的位置推断类型
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("Today, do {} pushups!", expensive_closure.value(intensity));
    println!("Next, do {} situps!", expensive_closure.value(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_closure.value(intensity)
      );
    }
  }
}

// 闭包会捕获环境
fn example() {
  let x = 4;

  let equals_to_x = |z| z == x; // x 在闭包可用

  // fn equals_to_x_fn(z: i32) -> bool { z == x } // 在函数不可用！

  let y = 4;

  assert!(equals_to_x(y));
}

// 闭包可以通过三种方式捕获其环境
// 1、获取所有权 FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
// 2、可变借用 FnMut 获取可变的借用值所以可以改变其环境
// 3、不可变借用 Fn 从其环境获取不可变的借用值

// 使用 move 关键词，让闭包获取到所使用的环境值的所有权
fn example1() {
  let x = vec![1, 2, 3];

  let equals_to_x = move |z| z == x;

  // println!("can't use x here: {:?}", x); // 这句会报错

  let y = vec![1, 2, 3];

  assert!(equals_to_x(y));
}

