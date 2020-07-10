// Rust 中每一个引用都有其 生命周期 lifetime ，也就是引用保持有效的作用域

// 借用检查器 borrow checker
// 下面演示的隐含的生命周期注解
// 变量 r 的生命周期 'a 比变量 x 的生命周期 'b 长很多
// rust 在编译时会比较变量的生命周期大小
// r 引用了一个生命周期比它自身小的对象，编译就会报错
// fn example() {
//   let r;                // ---------+-- 'a
//                         //          |
//   {                     //          |
//       let x = 5;        // -+-- 'b  |
//       r = &x;           //  |       |
//   }                     // -+       |
//                         //          |
//   println!("r: {}", r); //          |
// }                       // ---------+

fn main() {
  let s1 = String::from("abcd");
  let result;
  {
    let s2 = "xyz";

    result = longest(s1.as_str(), s2);
  }
  println!("The longest string is {}", result);
}

// 生命周期注解不会改变任何引用的生命周期长短
// 生命周期注解的意义在于告诉Rust多个引用的泛型生命周期参数是如何相互联系的
// 类似于泛型类型参数，泛型生命周期参数需要声明在函数名和参数列表之间的尖括号中
// 当具体的参数被传入时，'a 代表的是 x y 的生命中期中较小的哪一个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
