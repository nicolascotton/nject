use super::{Repository, User};
use std::sync::Mutex;

pub struct MemoryRepository {
    users: Mutex<Vec<User>>,
}

impl MemoryRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(Vec::new()),
        }
    }
}

impl Repository for MemoryRepository {
    fn create(&self, user: super::CreateUser) -> User {
        let mut users = self.users.lock().unwrap();
        let new_user = User {
            id: users.len(),
            name: user.name,
        };
        users.push(new_user.to_owned());
        new_user
    }

    fn get(&self, user_id: usize) -> Option<User> {
        let users = self.users.lock().unwrap();
        if let Some(user) = users.get(user_id) {
            Some(user.to_owned())
        } else {
            None
        }
    }
}
