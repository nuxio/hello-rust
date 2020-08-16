// 高级函数与闭包

fn add_one(x: i32) -> i32 {
  x + 1
}

// 接收函数作为参数, fn 被称为 函数指针，fn 是类型而不是trait
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

// 返回闭包
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

fn main() {
  let answer = do_twice(add_one, 6);

  println!("The answer is: {}", answer);

  let list_of_numbers = vec![1, 2, 3];
  let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string) // 像不像 JS 的传递函数。哈哈
    .collect();

  println!("list_of_strings is {:?}", list_of_strings);
}
