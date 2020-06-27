mod lua_userdata;
mod query_params;
mod query_result;

use crate::database::handle::DatabaseHandle;
use crate::database::compat::query_params::QueryParamIterator;
use crate::database::compat::query_result::QueryResult;
use crate::error::log_error;
use anyhow::Context;
use sqlx::Executor;
use tracing::trace;

#[derive(Clone)]
pub struct DatabaseCompat {
    handle: DatabaseHandle
}

impl DatabaseCompat {
    pub fn new(handle: DatabaseHandle) -> Self {
        Self {
            handle
        }
    }

    async fn query(&self, query_str: &str, params: QueryParamIterator) -> bool {
        trace!("Running compat database query: `{}` with params `{:?}`...", query_str, params.as_slice());

        let query = query_params::query_with(query_str, params);

        let result = query
            .execute(self.handle.pool())
            .await
            .with_context(|| format!("Failed to execute query: `{}`.", query_str));

        match result {
            Ok(_) => true,
            Err(e) => {
                log_error(e);
                false
            }
        }
    }

    async fn raw_query(&self, query_str: &str) -> bool {
        trace!("Running compat database rawQuery: `{}`...", query_str);

        let conn = self.handle.pool().acquire().await;

        let result = match conn {
            Ok(mut conn) => conn.execute(query_str).await,
            Err(e) => Err(e)
        };

        let result = result.with_context(|| {
            format!("Failed to execute unprepared query: `{}`.", query_str)
        });

        match result {
            Ok(_) => true,
            Err(e) => {
                log_error(e);
                false
            }
        }
    }

    async fn store_query(&self, query: String, params: QueryParamIterator) -> QueryResult {
        trace!("Running compat database storeQuery: `{}` with params `{:?}`...", query, params.as_slice());
        QueryResult::fetch(query, params, self.handle.pool()).await
    }

    async fn table_exists(&self, table: &str) -> bool {
        trace!("Checking if table exists: `{}`...", table);

        let result = sqlx::query!(r#"
                SELECT
                    COUNT(`table_name`) AS count
                FROM
                    `information_schema`.`tables`
                WHERE
                    `table_schema` = DATABASE() AND
                    `table_name`   = ?
                LIMIT 1;
            "#, table)
            .fetch_one(self.handle.pool())
            .await
            .context("Failed to query database table count.");

        match result {
            Ok(db_info) => db_info.count > 0,
            Err(e) => {
                log_error(e);
                false
            }
        }
    }
}
