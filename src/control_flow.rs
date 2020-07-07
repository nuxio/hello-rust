fn main() {
  let number = 3;

  // if 语句只接收 boolean 类型的值，如果不是会报错
  if number < 5 {
    println!("true");
  } else {
    println!("false");
  }

  let number = 6;

  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }

  // if 是一个表达式，可以用于赋值。。。
  let condition = true;
  let number = if condition { 5 } else { 9 }; // if else 表达式内返回值的类型需要一致，否则会报错

  println!("The value of number is: {}", number);

  // loop

  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2; // break 后的内容作为 loop 的结果返回
    }
  };

  println!("The result is {}", result);

  // while

  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  println!("LIFTOFF!!!");

  // for

  let a = [10, 20, 30, 40, 50, 60];

  for element in a.iter() {
    println!("The value is: {}", element);
  }

  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}
