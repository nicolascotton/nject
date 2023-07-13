use super::{
    models::{CreateUser, User},
    repository::UserRepository, errors::Error,
};
use nject::injectable;

#[injectable]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub async fn get(&self, user_id: i64) -> Result<User, Error> {
        self.repo.get(user_id).await
    }

    pub async fn create(&self, user: &CreateUser) -> Result<User, Error> {
        self.repo.create(user).await
    }

    pub async fn update(&self, user: &User) -> Result<(), Error> {
        self.repo.update(user).await
    }

    pub async fn delete(&self, user_id: i64) -> Result<(), Error> {
        self.repo.delete(user_id).await
    }
}
