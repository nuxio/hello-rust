#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

fn main() {
  // vector 是一种集合
  // Vector 只能存储相同类型的值
  // 长度可以变化，更类似于JS的数组
  // 一个 vector 内的元素在内存中是排列在一起的
  let v: Vec<i32> = Vec::new();
  let mut v = vec![1, 2, 3];

  v.push(5);
  // vector 还有 pop sort reverse 等常用方法

  let third: &i32 = &v[2];

  println!("The third element is {}", third);

  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
  }

  // let does_not_exist = &v[100]; // 超限访问，这种方式会在运行时报错
  let does_not_exist = v.get(100); // 这种方式不会，返回的是 None

  let first = &v[0];

  // push 会影响 vec 的长度
  // 如果没有足够的空间将所有元素依次相邻存放的情况下
  // 可能会要求分配新的内存并将老的元素拷贝到新的空间中
  // 这时，第一个元素的引用就指向了被释放的内存，导致bug
  // 解除注释会报错
  // v.push(6);

  println!("The first element is: {}", first);

  for i in &v {
    println!("{}", i);
  }

  let mut v = vec![100, 32, 55];

  for i in &mut v {
    *i += 50;
  }

  for i in &v {
    println!("{}", i);
  }

  // 通过定义枚举，可以在 Vector 中存储不同类型的数据
  // 相当于是用枚举将不同类型的值包装成同一个枚举类型
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];

  println!("Row {:?}", row);
}
