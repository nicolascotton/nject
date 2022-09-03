use crate::{
    models::{CreateUser, User},
    repository::Repository,
};
use nject::injectable;

#[injectable]
pub struct UserService<'a> {
    repository: &'a dyn Repository,
}

impl<'a> UserService<'a> {
    pub fn create(&self, user: CreateUser) -> User {
        self.repository.create(user)
    }

    pub fn get(&self, user_id: usize) -> Option<User> {
        self.repository.get(user_id)
    }
}
