// rust 中的面向对象编程

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> 是一个trait对象，它是Box中任何实现了Draw trait的类型的替身
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 不同于 trait bound ，泛型类型参数一次只能替代一个具体类型，而trait对象则允许在运行时替代多种具体类型
// 下面是trait bound的示例
// pub struct ScreenBound<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> ScreenBound<T>
//     where T: Draw {
//         pub fn run(&self) {
//             for component in self.components.iter() {
//                 component.draw();
//             }
//         }
//     }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // ...
        println!("Draw Button");
    }
}

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Draw SelectBox");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
