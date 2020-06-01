use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerStateError {
}

pub type Result<T> = std::result::Result<T, ServerStateError>;
