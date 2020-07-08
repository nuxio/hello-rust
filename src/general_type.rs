#[derive(Debug)]
struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn x(&self) -> &T {
    &self.x
  }

  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  // let number_list = vec![34, 50, 25, 100, 65];

  // let result = largest(&number_list);
  // println!("The largest number is {}", result);

  // let char_list = vec!['y', 'm', 'a', 'q'];

  // let result = largest(&char_list);
  // println!("The largest number is {}", result);

  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };

  let p3 = p1.mixup(p2);

  println!("p3 is: {:?}", p3);

  // 单态化（monomorphization）
  // rust 在编译时，编译器寻找所有泛型代码被调用的位置
  // 并使用泛型代码针对具体类型生成代码
  // 例如
  let integer = Some(5);
  let float = Some(5.0);
  // 单态化后的的代码看起来像这样：
  enum Option_i32 {
    Some(i32),
    None,
  }

  enum Option_f64 {
    Some(f64),
    None,
  }

  let integer = Option_i32::Some(5);
  let float = Option_f64::Some(5.0);
}

// fn largest<T>(list: &[T]) -> T {
//   let mut largest = list[0];

//   for &item in list {
//     if item > largest {
//       largest = item;
//     }
//   }

//   largest
// }
