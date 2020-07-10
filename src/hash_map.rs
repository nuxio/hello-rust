use std::collections::HashMap;

fn main() {
  // hash map 是一种集合，它通过一个哈希函数来实现映射，决定如何将键和值放入内存中
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 5);

  // 通过 Vec collect 创建
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 5];

  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value); // field_name 和 field_value 的所有权会被获取
  // 这里 field_name 和 field_value 不再有效

  let team_name = String::from("Blue");
  let score = scores.get(&team_name); // Some(10)

  println!("Blue score is {:?}", score);

  // 遍历
  for (key, value) in &scores {
    println!("{}, {}", key, value);
  }

  // 更新
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 50); // 覆盖上一个值
  scores.insert(String::from("Yellow"), 80);

  println!("{:?}", scores);

  // 检查是否存在，不存在则插入值，存在则不插入
  scores.entry(String::from("Yellow")).or_insert(90);
  scores.entry(String::from("Orange")).or_insert(90);

  println!("{:?}", scores);

  // or_insert 方法会返回键所对应的值的可变引用 &mut V
  let text = "hello world wonderful world";
  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0); // 返回 &mut V
    *count += 1; // 、*count 解引用
  }

  println!("{:?}", map);
}
