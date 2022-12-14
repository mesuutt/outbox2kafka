use rdkafka::error::KafkaError;
use thiserror::Error as ThisError;
use uuid::Uuid;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, ThisError)]
pub enum AppError {
    #[error("An error occurred at db: {0}")]
    DBError(String),

    #[error("An error occurred while parsing duration: {0}")]
    DurationParseError(String),

    #[error("Kafka error: {0}")]
    KafkaError(#[from] KafkaError),

    #[error("metadata of record is invalid. It must be json deserializable string: {0}")]
    InvalidMetadataError(Uuid),
}

impl From<sqlx::Error> for AppError {
    fn from(sqlx_error: sqlx::Error) -> Self {
        match sqlx_error.as_database_error() {
            Some(db_error) => AppError::DBError(db_error.to_string()),
            None => AppError::DBError(String::from("Unrecognized database error!")),
        }
    }
}
