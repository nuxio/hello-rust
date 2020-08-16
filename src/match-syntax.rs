// 模式的所有语法

fn main() {
    // 匹配字面值
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配命名变量
    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 多个模式
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 范围匹配
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 解构
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;

    println!("x is {}, y is {}", x, y);

    let p = Point { x: 10, y: 70 };
    let Point { x: a, y: b } = p;

    println!("a is {}, b is {}", a, b);

    let p = Point { x: 0, y: 80 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x), // 匹配 y 为 0 时 x 的值
        Point { x: 0, y } => println!("On the y axis at {}", y), // 匹配 x 为 0 时 y 的值
        Point { x, y } => println!("On neither axis: ({}, {})", x, y), // 匹配 x, y 的值
    }

    // 解构枚举
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // let msg = Message::ChangeColor(0, 160, 255);

    // match msg {
    //     Message::Quit => {
    //         println!("The Quit variant has no data to destructure.")
    //     },
    //     Message::Move { x, y } => {
    //         println!(
    //             "Move in the x direction {} and in the y direction {}",
    //             x,
    //             y
    //         );
    //     },
    //     Message::Write(text) => println!("Text message: {}", text),
    //     Message::ChangeColor(r, g, b) => {
    //         println!(
    //             "Change the color to red {}, green {}, and blue {}",
    //             r,
    //             g,
    //             b
    //         )
    //     }
    // }

    // 解构嵌套的结构体和枚举
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        },
        _ => (),
    }

    // 解构结构体和元组
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

    println!("feet is {}, inches is {}, x is {}, y is {}", feet, inches, x, y);

    // 使用 _ 忽略值
    let _x = 233; // 忽略变量

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { // 忽略内部值
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => { // 忽略部分
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    let s = Some(String::from("Hello!"));

    if let Some(_) = s { // _ 不会获取 s 的所有权
        println!("found a string");
    }

    println!("{:?}", s);

    // 使用 .. 忽略剩余值
    struct PointX {
        x: i32,
        y: i32,
        z: i32,
    }
    
    let origin = PointX { x: 0, y: 0, z: 0 };
    
    match origin {
        PointX { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

    // 匹配守卫
    let num = Some(6);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), // 增强匹配规则
        Some(x) => println!("{}", x),
        None => (),
    }

    // @ 绑定
    enum MessageB {
        Hello { id: i32 },
    }
    
    let msg = MessageB::Hello { id: 5 };
    
    match msg {
        MessageB::Hello { id: id_variable @ 3..=7 } => { // 匹配值的同时，绑定
            println!("Found an id in range: {}", id_variable)
        },
        MessageB::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        MessageB::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}
