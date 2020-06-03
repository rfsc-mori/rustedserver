use super::ConnectionStringOptions;
use anyhow::{Context, Result};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use url::Url;

macro_rules! percent_encoding_set {
    ($base:expr; $($delim:expr),*) => {
        &$base$(.add($delim))*;
    }
}

const URI_RESERVED: &AsciiSet = percent_encoding_set!(
    CONTROLS;

    // https://tools.ietf.org/html/rfc3986#section-2.2
    /* gen-delims */
    b':', b'/', b'?', b'#', b'[', b']', b'@',

    /* sub-delims */
    b'!', b'$', b'&', b'\'', b'(', b')', b'*', b'+', b',', b';', b'='
);

fn uri_encode(text: String) -> String {
    utf8_percent_encode(text.as_str(), URI_RESERVED).to_string()
}

pub fn build_connection_string(options: ConnectionStringOptions) -> Result<String> {
    let mut url = String::new();

    if let Some(scheme) = options.scheme {
        url.push_str(scheme.as_str());
    }

    url.push_str("://");

    let mut authority = String::new();

    if let Some(user) = options.user {
        authority.push_str(uri_encode(user).as_str());

        if let Some(password) = options.password {
            authority.push_str(":");
            authority.push_str(uri_encode(password).as_str());
        }

        authority.push_str("@");
    }

    if let Some(host) = options.host {
        authority.push_str(uri_encode(host).as_str());
    }

    if let Some(port) = options.port {
        authority.push_str(":");
        authority.push_str(port.to_string().as_str());
    }

    url.push_str(authority.as_str());

    if let Some(database) = options.database {
        url.push_str("/");
        url.push_str(uri_encode(database).as_str());
    }

    let mut query = String::new();

    for (key, value) in options.query {
        if !query.is_empty() {
            query.push_str("&");
        }

        query.push_str(uri_encode(key).as_str());
        query.push_str("=");
        query.push_str(uri_encode(value).as_str());
    }

    if !query.is_empty() {
        url.push_str(query.as_str());
    }

    let url = Url::parse(url.as_str())
        .context("Failed to validate the generated connection URL.")?;

    Ok(url.into_string())
}
