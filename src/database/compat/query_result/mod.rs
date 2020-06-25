mod lua_userdata;

use crate::database::compat::query_params;
use crate::database::compat::query_params::QueryParamIterator;
use crate::error::log_error;
use anyhow::format_err;
use futures::StreamExt;
use futures::stream::BoxStream;
use owning_ref::OwningHandle;
use sqlx::{MySql, Row, Type};
use sqlx::decode::Decode;
use sqlx::mysql::{MySqlPool, MySqlRow};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

type SqlRowStream<'q> = BoxStream<'q, std::result::Result<MySqlRow, sqlx::Error>>;

#[derive(Clone)]
pub struct QueryResult {
    cursor: Arc<OwningHandle<String, Arc<Mutex<Option<SqlRowStream<'static>>>>>>,
    row: Arc<RwLock<Option<MySqlRow>>>
}

impl QueryResult {
    #[allow(unsafe_code)] // OwningHandle's new_with_fn.
    #[allow(clippy::option_expect_used)] // Panic on new_with_fn with invalid ptr.
    pub async fn fetch(query_str: String, params: QueryParamIterator, pool: &MySqlPool) -> Self {
        // This unsafe may be removed in future if/when SQLx query() accepts owned Strings.
        let cursor = {
            let cursor = OwningHandle::new_with_fn(query_str, |query_ptr| {
                let query_str = unsafe { query_ptr.as_ref() }
                    .expect("Failed to unwrap OwningHandle's raw pointer.");

                let query = query_params::query_with(query_str, params);
                let results = query.fetch(pool);

                Arc::new(Mutex::new(Some(results)))
            });

            Arc::new(cursor)
        };

        let row = Arc::new(RwLock::new(None));

        let result = Self {
            cursor,
            row
        };

        result.fetch_next().await;

        result
    }

    async fn get<'r, T>(&self, row: Option<&'r MySqlRow>, column: &str) -> T
    where
        T: Default + Decode<'r, MySql> + Type<MySql>,
    {
        match row {
            Some(row) => match row.try_get(column) {
                Ok(value) => value,
                Err(e) => {
                    log_error(format_err!(e));
                    T::default()
                }
            },
            None => T::default()
        }
    }

    async fn get_number(&self, column: &str) -> mlua::Integer {
        let row = self.row.read().await;
        self.get(row.as_ref(), column).await
    }

    async fn get_string(&self, column: &str) -> String {
        let row = self.row.read().await;
        self.get(row.as_ref(), column).await
    }

    async fn get_stream(&self, column: &str) -> Vec<u8> {
        let row = self.row.read().await;
        self.get(row.as_ref(), column).await
    }

    pub async fn has_next(&self) -> bool {
        self.row.read().await.is_some()
    }

    async fn fetch_next(&self) -> bool {
        let mut cursor = self.cursor.lock().await;

        let row = match cursor.as_mut() {
            Some(row) => match row.next().await {
                Some(row) => match row {
                    Ok(row) => Some(row),
                    Err(e) => {
                        log_error(format_err!(e));
                        None
                    }
                },
                None => {
                    *cursor = None;
                    None
                }
            },
            None => None
        };

        let mut current_row = self.row.write().await;
        *current_row = row;

        current_row.is_some()
    }

    async fn free(&self) -> bool {
        *self.row.write().await = None;
        self.cursor.lock().await.take().is_some()
    }
}
