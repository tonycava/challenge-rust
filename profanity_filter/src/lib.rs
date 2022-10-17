pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Message {
        Message {
            content,
            user,
        }
    }
    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            return None;
        }
        Some(&self.content)
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    if ms.send_ms() == None {
        return (false, "ERROR: illegal")
    }
    (true, &ms.content)
}