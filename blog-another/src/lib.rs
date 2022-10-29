pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    
    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approved_num: 1,
        }
    }
}

pub enum PendingResult {
    Pending(PendingReviewPost),
    Approved(Post),
}

impl PendingResult {
    pub fn check_if_approved(&self) -> bool {
        match self {
            PendingResult::Pending(_) => false,
            PendingResult::Approved(_) => true,
        }
    }

    pub fn detach(self) -> Post {
        match self {
            PendingResult::Pending(_) => panic!("Illegal Operation!"),
            PendingResult::Approved(s) => s,
        }
    }

    pub fn approve(self) -> PendingResult {
        match self {
            PendingResult::Pending(s) => s.approve(),
            PendingResult::Approved(_) => self,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approved_num: i32,
}

impl PendingReviewPost {
    pub fn approve(self) -> PendingResult {
        if self.approved_num != 2 {
            PendingResult::Pending(PendingReviewPost{
                content: self.content,
                approved_num: self.approved_num + 1
            })
        } else {
            PendingResult::Approved(Post{ content: self.content })
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
