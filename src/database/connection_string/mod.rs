mod mysql;

use anyhow::{Context, Result};

#[derive(Debug, Default)]
pub struct ConnectionStringOptions {
    pub scheme: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database: Option<String>,
    pub query: Vec<(String, String)>,
}

#[derive(Debug, Default)]
pub struct ConnectionStringBuilder {
    options: ConnectionStringOptions,
}

impl ConnectionStringBuilder {
    pub fn new() -> ConnectionStringBuilder {
        ConnectionStringBuilder::default()
    }

    #[allow(dead_code)] // optional
    pub fn scheme(mut self, scheme: String) -> Self {
        if !scheme.is_empty() {
            self.options.scheme = Some(scheme);
        }
        self
    }

    #[allow(dead_code)] // optional
    pub fn user(mut self, user: String) -> Self {
        if !user.is_empty() {
            self.options.user = Some(user);
        }
        self
    }

    #[allow(dead_code)] // optional
    pub fn password(mut self, password: String) -> Self {
        if !password.is_empty() {
            self.options.password = Some(password);
        }
        self
    }

    #[allow(dead_code)] // optional
    pub fn host(mut self, host: String) -> Self {
        if !host.is_empty() {
            self.options.host = Some(host);
        }
        self
    }

    #[allow(dead_code)] // optional
    pub fn port(mut self, port: u16) -> Self {
        self.options.port = Some(port);
        self
    }

    #[allow(dead_code)] // optional
    pub fn database(mut self, database: String) -> Self {
        if !database.is_empty() {
            self.options.database = Some(database);
        }
        self
    }

    #[allow(dead_code)] // optional
    pub fn add_query(mut self, key: String, value: String) -> Self {
        if !key.is_empty() {
            self.options.query.push((key, value));
        }
        self
    }

    pub fn build(self) -> Result<String> {
        let url = mysql::build_connection_string(self.options)
            .context("Failed to build the mysql connection string.")?;

        Ok(url)
    }
}
