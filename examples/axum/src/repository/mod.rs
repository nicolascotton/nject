use crate::models::{CreateUser, User};

pub mod memory;

pub trait Repository {
    fn create(&self, user: CreateUser) -> User;
    fn get(&self, user_id: usize) -> Option<User>;
}
