struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someone"),
    active: true,
    sign_in_count: 1,
  };

  println!("User1 email: {}", user1.email);

  let user2 = build_user(String::from("somebody@example.com"), String::from("somebody"));

  println!("User2 email: {}", user2.email);

  let user3 = User {
    email: String::from("someday@example.com"),
    username: String::from("someday"),
    ..user1 // 和js解构类似，不过只取 User 结构除了 email 和 username 的剩余属性
  };

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  println!("Color black r is: {}", black.0);
  println!("Point origin y is: {}", black.1);
}

fn build_user(email: String, username: String) -> User {
  User {
    email, // 和 js 一样的简写，同 email: email,
    username,
    active: true,
    sign_in_count: 1,
  }
}