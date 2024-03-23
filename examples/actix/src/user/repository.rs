use super::{
    errors::Error,
    models::{CreateUser, User},
};
use nject::injectable;
use sqlx::{query, query_as};

#[injectable]
pub struct UserRepository {
    pool: super::Pool,
}

impl UserRepository {
    pub async fn get(&self, id: i64) -> Result<User, Error> {
        let (id, name) = query_as("SELECT rowid, name FROM user WHERE rowid = ?")
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(User { id, name })
    }
    pub async fn update(&self, user: &User) -> Result<(), Error> {
        let result = query("UPDATE user SET name = ? WHERE rowid = ?")
            .bind(&user.name)
            .bind(user.id)
            .execute(&*self.pool)
            .await?;
        if result.rows_affected() == 0 {
            return Err(Error::NotFound);
        }
        Ok(())
    }
    pub async fn delete(&self, id: i64) -> Result<(), Error> {
        query("DELETE FROM user WHERE rowid = ?")
            .bind(id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    pub async fn create(&self, user: &CreateUser) -> Result<User, Error> {
        let (id, name) = query_as("INSERT INTO user (name) VALUES (?) RETURNING rowid, name")
            .bind(&user.name)
            .fetch_one(&*self.pool)
            .await?;
        Ok(User { id, name })
    }
}
