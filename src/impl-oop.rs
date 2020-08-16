trait State {
  // self: Box<Self> 获取Box<Self> 的所有权，以便返回需要的数据类型
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, post: &'a Post) -> &'a str {
      ""
  }
}

struct Draft {}

impl State for Draft {
  fn request_review (self: Box<Self>) -> Box<dyn State> {
      Box::new(PendingReview {})
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
      self
  }
}

struct PendingReview {}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
      Box::new(Published {})
  }
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
      self
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
      &post.content
  }
}

pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Post {
      Post {
          state: Some(Box::new(Draft {})),
          content: String::new(),
      }
  }

  pub fn add_text(&mut self, text: &str) {
      self.content.push_str(text);
  }

  pub fn content(&self) -> &str {
      self.state.as_ref().unwrap().content(self)
  }

  fn request_review(&mut self) {
      if let Some(s) = self.state.take() {
          self.state = Some(s.request_review());
      }
  }

  pub fn approve(&mut self) {
      if let Some(s) = self.state.take() {
          self.state = Some(s.approve())
      }
  }
}


fn main() {
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.request_review();

  println!("The content of post is: {}", post.content());

  post.approve();

  println!("The content of post is: {}", post.content());
}
