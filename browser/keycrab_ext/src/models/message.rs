use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SendMessageArgs<'a> {
    command: &'a str,
    username: &'a str,
    password: &'a str,
}

impl<'a> SendMessageArgs<'a> {
    pub fn new(command: &'a str, username: &'a str, password: &'a str) -> Self {
        Self {
            command,
            username,
            password,
        }
    }
}
