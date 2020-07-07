fn main() {
  let mut s = String::new();

  let data = "initial contents";
  let s = data.to_string();
  let s = "initial contents".to_string();
  let s = String::from("initial contents");
  let s = String::from(data);

  let mut s1 = String::from("foo");
  let s2 = "bar";

  s1.push_str(s2); // 加入字符串
  s1.push('x'); // 加入单个字符
  println!("s1 is {}, s2 is {}", s1, s2);

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");

  // + 运算符调用了 add 函数，并不像js中那么简单
  // 必须是 String + &str 类型
  // 这里 &s2 的类型是 &String，编译通过的原因是 Rust 强制将 &String 转成了 &str
  // 这个转换叫 解引用强制多态 deref coercion
  let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
                     // add 函数获取了 s1 的所有权，附加上从 s2 中拷贝的内容，并返回结果的所有权

  println!("s3 is {}", s3);

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  // 多个字符串相加，就很繁琐
  let s = s1 + "-" + &s2 + "-" + &s3;

  println!("s is {}", s);

  // 使用 format! 宏，不会获取任何参数的所有权
  let s = format!("{}-{}-{}", s, s2, s3);

  println!("s is {}", s);

  // rust 字符串不支持索引，以下代码会报错
  // let h = s1[0];

  // String 是一个 Vec<u8> 的封装
  // 所以每位字符的最大存储能力是 8bit 也就是 255
  // Rust采用UTF-8编码，除了英文字符和一些符号外，其它语言均需要2个字节及以上的空间存储
  let len = String::from("Hola").len(); // 4
  let len = String::from("Здравствуйте").len(); // 24

  let hello = "Здравствуйте";
  // 索引 0 指向的是字节位置，即第一个字节所存储的内容
  // 但是由于字符串的第一个字符是两个字节组成的，只取第一个字节无法返回预期的结果
  // rust 对这种情况会直接抛出异常
  // let answer = &hello[0];

  // 这里取到的是前两个字符
  let s = &hello[0..4];

  // 如果不信邪，像下面这样操作，会导致运行时错误
  // let s = &hello[0..1];

  println!("s is {}", s);

  // 遍历字符串的 char
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  // 遍历字符串的原始字节
  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}
