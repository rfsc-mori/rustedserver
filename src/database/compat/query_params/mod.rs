mod lua_convert;

use bstr::BString;
use sqlx::MySql;
use sqlx::mysql::MySqlArguments;
use sqlx::query::Query;
use std::vec;

pub type QueryParamIterator = vec::IntoIter<QueryParams>;

#[derive(Debug)]
pub enum QueryParams {
    Boolean(bool),
    Integer(mlua::Integer),
    Number(mlua::Number),
    String(BString)
}

// TODO: Error on param count mismatch: waiting for SQLx fix.
pub fn query_with(query_str: &str, params: QueryParamIterator) -> Query<MySql, MySqlArguments> {
    let query = sqlx::query(query_str);
    params.fold(query, |query, param| param.bind_to(query))
}

impl QueryParams {
    fn bind_to(self, query: Query<MySql, MySqlArguments>) -> Query<MySql, MySqlArguments> {
        match self {
            Self::Boolean(b) => query.bind(b),
            Self::Integer(i) => query.bind(i),
            Self::Number(n) => query.bind(n),
            Self::String(s)  => query.bind(s.to_vec())
        }
    }
}
