use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::fmt;
use actix_session::{SessionGetError, SessionInsertError};
use tera::Error as TeraError;

#[derive(Debug)]
pub enum AppError {
    InvalidCredentials,
    UsernameExists,
    DatabaseError(sqlx::Error),
    SessionInsertError(SessionInsertError),
    SessionGetError(SessionGetError),
    TeraError(TeraError),
    InternalError(String),

    NotFound(String),      
    BadRequest(String),    
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::DatabaseError(error)
    }
}

impl From<SessionInsertError> for AppError {
    fn from(error: SessionInsertError) -> Self {
        AppError::SessionInsertError(error)
    }
}

impl From<SessionGetError> for AppError {
    fn from(error: SessionGetError) -> Self {
        AppError::SessionGetError(error)
    }
}

impl From<TeraError> for AppError {
    fn from(error: TeraError) -> Self {
        AppError::TeraError(error)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidCredentials => write!(f, "Username atau password salah."),
            AppError::UsernameExists => write!(f, "Username tersebut sudah digunakan."),
            AppError::NotFound(msg) => write!(f, "{}", msg),
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::InternalError(msg) => write!(f, "Terjadi kesalahan: {}", msg),
            _ => write!(f, "Terjadi kesalahan internal pada server."),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::UsernameExists => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(e) => tracing::error!("Database error: {:?}", e),
            AppError::SessionInsertError(e) => tracing::error!("Session Insert error: {:?}", e),
            AppError::SessionGetError(e) => tracing::error!("Session Get error: {:?}", e),
            AppError::TeraError(e) => tracing::error!("Tera template error: {:?}", e),
            AppError::InternalError(e) => tracing::error!("Internal error: {}", e),
            AppError::NotFound(msg) => tracing::warn!("Not Found: {}", msg),
            AppError::BadRequest(msg) => tracing::warn!("Bad Request: {}", msg),
            _ => (),
        }

        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}