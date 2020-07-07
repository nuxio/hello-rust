fn main() {
  let mut s1 = String::from("hello");

  let len = calculate_length(&s1); // 传入s1的引用

  println!("The length of '{}' is {}.", s1, len);

  change(&mut s1); // 传入可修改的引用

  // 同一个作用域内，对同一个变量只能有一个可修改的引用，而且不能**同时**使用不可变引用和可变引用
  let mut s = String::from("hello");

  // let r1 = &s; // no problem
  // let r2 = &s; // no problem
  // let r3 = &mut s; // BIG PROBLEM

  // println!("{}, {}, and {}", r1, r2, r3); // important! 因为这里使用了 r1 r2
  // r3 可能会变更 s 的值，导致r1, r2引用到的值并不是预期的值

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{} and {}", r1, r2);
  // r1 and r2 在此后没有再被使用

  let r3 = &mut s; // no problem
  println!("{}", r3);
}

fn calculate_length(s: &String) -> usize { // s 是一个引用
  s.len()  // 返回s的长度
}

fn change(some_thing: &mut String) {
  some_thing.push_str(", world!");
}
