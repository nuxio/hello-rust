#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// implement
impl Rectangle {
  fn area(&self) -> u32 {
    self.height * self.width
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

// 可以多次 impl,与合在一起是一样的
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      height: size,
      width: size,
    }
  }
}

// 结构 struct 的意义之一，就是明确变量的意思
fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    area(&rect1)
  );

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  println!("rect1 is {:?}", rect1);
  println!("rect1 is {:#?}", rect1);

  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };
  let rect3 = Rectangle::square(32);

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
