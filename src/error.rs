use rusqlite::Error as RusqliteError;
use serde_json::Error as SerdeError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TodoError {
    Io(io::Error),
    SerdeJson(SerdeError),
    Rusqlite(RusqliteError),
    InvalidJsonFormat,
    DatabaseError(String),
    NotFound(String),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TodoError::Io(ref err) => write!(f, "Error de entrada/salida: {}", err),
            TodoError::SerdeJson(ref err) => write!(f, "Error de JSON: {}", err),
            TodoError::Rusqlite(ref err) => write!(f, "Error de base de datos: {}", err),
            TodoError::InvalidJsonFormat => write!(
                f,
                "Formato JSON inválido. Asegúrese de que el archivo tenga el formato adecuado."
            ),
            TodoError::DatabaseError(ref msg) => write!(f, "Error en la base de datos: {}", msg),
            TodoError::NotFound(ref msg) => write!(f, "No encontrado: {}", msg),
        }
    }
}

impl From<io::Error> for TodoError {
    fn from(err: io::Error) -> TodoError {
        TodoError::Io(err)
    }
}

impl From<SerdeError> for TodoError {
    fn from(err: SerdeError) -> TodoError {
        TodoError::SerdeJson(err)
    }
}

impl From<RusqliteError> for TodoError {
    fn from(err: RusqliteError) -> TodoError {
        TodoError::Rusqlite(err)
    }
}
