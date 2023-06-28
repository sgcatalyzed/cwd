use std::str::FromStr;

use serde::{de::DeserializeOwned, Serialize};
use tendermint_rpc::{Client, HttpClient, Url};
use tracing::error;

use crate::{ClientConfig, Error, Result};

pub fn create_http_client(
    node: Option<&String>,
    client_cfg: &ClientConfig,
) -> Result<HttpClient> {
    let url_str = node.unwrap_or(&client_cfg.node);
    let url = Url::from_str(url_str)?;
    HttpClient::new(url).map_err(Into::into)
}

pub async fn do_abci_query<Q: Serialize, R: DeserializeOwned>(
    client: &HttpClient,
    query: Q,
) -> Result<R> {
    // serialize the query into binary
    let query_bytes = serde_json::to_vec(&query)?;

    // do query
    // must use "app" path
    let result = client.abci_query(Some("app".into()), query_bytes, None, false).await?;

    if result.code.is_err() {
        return Err(Error::query_failed(result.log));
    }

    // deserialize the response
    // we expect that the response is a valid JSON string.
    // throw an error if this is not the case.
    match serde_json::from_slice(&result.value) {
        Ok(response) => Ok(response),
        Err(err) => {
            match String::from_utf8(result.value.clone()) {
                Ok(response_str) => {
                    error!(
                        target: "ABCI query is successful but the response is not valid JSON",
                        response = response_str,
                    );
                },
                Err(_) => {
                    let response_raw = hex::encode(&result.value);
                    error!(
                        target: "ABCI query is successful but the response is not valid JSON or UTF8",
                        response_raw,
                    );
                },
            }
            Err(err.into())
        },
    }
}
