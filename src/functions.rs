fn main() {
    println!("Hello world!");

    another_function(5, 6);

    // 声明（statement）没有返回值
    let x = plus_one(five());

    // 块，返回最后一个表达式（expression）的值
    let y = {
      let x = 3;
      x + 1 // 没有分号结尾，是一个表达式，不是一个声明。如果有分号结尾，就不会返回值了
    };

    println!("The value of x is: {}, y is: {}", x , y);
}

fn another_function(x: i32, y: i32) {
    println!("Another function. x is: {}, y is: {}", x, y);
}

// 函数返回值可以显式的使用return关键词，也可以和块一样，隐式的返回最后一个表达式的值
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}