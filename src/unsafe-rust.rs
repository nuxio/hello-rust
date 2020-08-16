// unsafe 代码是在 rust 无法确认其安全性的情况下，程序员确信不会有问题，则使用 unsafe 标记，”绕开“rust验证

use std::slice;

// 使用 extern 创建和使用 外部函数接口（Foreign Function Interface， FFI）
// 这里使用了 c 语言的 应用二进制接口（application binary interface，ABI）
extern "C" {
  fn abs(input: i32) -> i32;
}

// no_mangle 表示在编译阶段不要混淆此函数的名称，不然外部语言就调用不到了
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// 访问和修改可变静态变量都是 不安全 的
// 因为拥有可以全局访问的可变数据，难以保证不存在数据竞争
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 当至少有一个方法中包含编译器不能验证的情况时 trait 是不安全的
unsafe trait Foo {
  // methods go here
}

unsafe impl Foo for i32 {
  // method implementations go here
}

fn main() {
  let mut num = 5;

  let r1 = &num as *const i32; // 裸指针
  let r2 = &mut num as *mut i32;

  // unsafe 代码块，切换模式
  unsafe {
    println!("r1 is: {}", *r1); // 解引用裸指针
    println!("r2 is: {}", *r2);
  }

  unsafe {
    dangerous(); // 调用不安全的函数，需要在 unsafe 块中调用
  }

  let mut v = vec![1, 2, 3, 4, 5, 6];

  let r = &mut v[..];

  let (a, b) = split_at_mut(r, 3);

  println!("a is {:?}, b is {:?}", a, b);

  // extern 块中声明的函数在 Rust 代码中总是不安全的
  // 因为其他语言不会强制执行 Rust 的规则且 Rust 无法检查它们
  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }
}

unsafe fn dangerous() {}

// 不安全代码包含在unsafe代码块中的函数不用标记 unsafe
fn split_at_mut(ss: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = ss.len();
  let ptr = ss.as_mut_ptr(); // 返回 slice 的裸指针

  assert!(mid <= len);

  unsafe {
    (
      slice::from_raw_parts_mut(ptr, mid), // 从 ptr 中创建了一个有 mid 个项的 slice
      slice::from_raw_parts_mut(ptr.add(mid), len - mid), // 在 ptr 上调用 add 方法并使用 mid 作为参数来获取一个从 mid 开始的裸指针
    )
  }
}
