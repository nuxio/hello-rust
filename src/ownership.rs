fn main() {
  // 简单类型的数据是复制操作
  let x = 5;
  let y = x;

  println!("x = {}, y = {}", x, y);

  // 复杂类型的数据会进行move操作，而不是复制操作
  let s1 = String::from("Hello");
  let s2 = s1; // move

  // 这里再引用s1会报错，s1已经被move到了s2
  // println!("{}, world!", s1);
  println!("{}, world!", s2);

  // 如果确实想要复制一份值，有单独的方法
  // clone 操作比较昂贵
  let s3 = s2.clone();

  println!("{}, world!", s3);

  let s = String::from("Hello");

  takes_ownership(s); // 函数调用，将变量传递给函数参数，与赋值给变量一致
                      // s 在这里已经被move走了，不能访问

  let x = 6;

  makes_copy(x);


  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
} // some_string 超出作用域，`drop`方法调用，释放内存

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String

  (s, length)
}
