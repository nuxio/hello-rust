// slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合
// slice 没有所有权
fn main() {
  let s = String::from("hello");

  let len = s.len();

  let slice = &s[0..len];
  let slice = &s[..]; // same

  println!("{}", slice);

  // 字符串字面量就是 slice
  let s = "Hello world!"; // s 的类型就是 &str，是一个不可变引用

  // 数组也可以有slice
  let a = [1, 2, 3, 4, 5];
  let slice = &a[1..3]; // 这个 slice 的类型是 &[i32]，数组slice的工作方式和字符串一样，都是保存的引用
}

// 字符串 slice 的类型声明写作 &str
fn first_word(s: &String) -> &str {
  // &String 表示参数为字符串的引用
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i]; // 字符串 slice，这里将是一个包含指向 s 第 0 个字节的指针和长度值为 i 的 slice
    }
  }

  &s[..]
}
