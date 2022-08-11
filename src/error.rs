use thiserror::Error as ThisError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Clone, Debug, ThisError)]
pub enum AppError {
    #[error("An error occurred at db: {0}")]
    DBError(String),
    #[error("An error occurred while parsing cli options: {0}")]
    CLIParseError(String),
}

impl From<sqlx::Error> for AppError {
    fn from(sqlx_error: sqlx::Error) -> Self {
        match sqlx_error.as_database_error() {
            Some(db_error) => AppError::DBError(db_error.to_string()),
            None => {
                AppError::DBError(String::from("Unrecognized database error!"))
            }
        }
    }
}