pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approved_num: i32,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approved_num: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state.as_ref().unwrap().add_text(&mut self.content, text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject(&mut self.approved_num));
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(&mut self.approved_num));
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>, _: &mut i32) -> Box<dyn State>;
    fn reject(self: Box<Self>, _: &mut i32) -> Box<dyn State>;
    fn content<'a>(&self, _: &'a Post) -> &'a str { "" }
    fn add_text(&self, _: &mut String, _: &str) {}
}

struct Draft {}

impl State for Draft {
    fn approve(self: Box<Self>, _: &mut i32) -> Box<dyn State> { self }
    fn reject(self: Box<Self>, _: &mut i32) -> Box<dyn State> { self }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn add_text(&self, content: &mut String, text: &str) {
        content.push_str(text);
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }

    fn approve(self: Box<Self>, approved_num: &mut i32) -> Box<dyn State> {
        *approved_num += 1;
        if *approved_num == 2 {
            Box::new(Published {})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>, approved_num: &mut i32) -> Box<dyn State> {
        *approved_num = 0;
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>, _: &mut i32) -> Box<dyn State> { self }
    fn reject(self: Box<Self>, _: &mut i32) -> Box<dyn State> { self }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
