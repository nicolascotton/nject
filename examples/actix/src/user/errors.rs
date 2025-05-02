use serde::Serialize;

#[derive(Serialize)]
pub enum Error {
    NotFound,
    Unexpected,
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::Unexpected,
        }
    }
}
