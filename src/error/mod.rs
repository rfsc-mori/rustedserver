use anyhow::{format_err, Error};
use std::borrow::Borrow;
use std::fmt;
use tokio::sync::mpsc;
use tracing::error;

pub type ErrorReceiver = mpsc::UnboundedReceiver<Error>;
pub type ErrorSender = mpsc::UnboundedSender<Error>;

macro_rules! log_errors {
    ($($result:expr),*) => {
        use anyhow::format_err;

        $(if let Err(e) = $result {
            log_error(format_err!(e))
        })

        *
    };

    {$($result:expr);* $(;)?} => {
        log_errors!($($result),*);
    }
}

pub fn log_error<E: Borrow<Error> + fmt::Display>(e: E) {
    error!("Error: {}", e);

    for source in e.borrow().chain().skip(1) {
        error!("Caused by: {}", source);
    }
}

pub fn from_lua_err(e: mlua::Error) -> Error {
    match e {
        mlua::Error::CallbackError { traceback, cause } => {
            format_err!("Lua CallbackError:\n{0}\n\nCaused by:\n{1}", traceback, cause)
        },
        _ => format_err!("Lua {}", e)
    }
}

pub fn create_error_channel() -> (ErrorSender, ErrorReceiver) {
    mpsc::unbounded_channel()
}
