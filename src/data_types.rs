/**
 * 数据类型
 * */

fn main() {
    // ------------------- Scalar type -------------------

    // integer
    
    let x: u8 = 2;
    let x: i8 = 127;
    let x: u16 = 22;
    let x: i16 = 22;
    let x: u32 = 22;
    let x: i32 = 22;
    let x: u64 = 22;
    let x: i64 = 22;
    let x: u128 = 22;
    let x: i128 = 22;

    // float
    let y: f32 = 60.8;
    let y: f64 = 60.8;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // 元组，长度固定，不能改变，元素类型可以不一致
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", tup.0);
    println!("The value of z is: {}", tup.2);

    // array 数组长度固定，不能改变，且所有元素的类型必须一致
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // same as: let a = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];

    let index = 10;

    let element = a[index]; // 访问超出数组长度的数据，编译不会报错，运行时会报错

    println!("The value of element is: {}", element);
}
