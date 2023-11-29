//! WebSocket client for [polygon.io](https://polygon.io).
//!
//! # Authentication
//!
//! Use an [API key](https://polygon.io/dashboard/api-keys) to authenticate.
//! This can be provided through the `auth_key` parameter to
//! [`WebSocketClient::new()`] or through the `POLYGON_AUTH_KEY` environment variable.
//!
//! # Example
//!
//! ```
//! use polygon_client::websocket::{STOCKS_CLUSTER, WebSocketClient};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = WebSocketClient::new(STOCKS_CLUSTER, None);
//!     let res = client.receive();
//!     let msg_text = res.unwrap().into_text().unwrap();
//!     println!("msg: {}", msg_text);
//! }
//! ```
use std::env::{self, VarError};

use futures_util::{SinkExt, StreamExt};
use thiserror::Error as ThisError;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::{ParseError, Url};

pub const STOCKS_CLUSTER: &str = "stocks";
pub const FOREX_CLUSTER: &str = "forex";
pub const CRYPTO_CLUSTER: &str = "crypto";

pub struct WebSocketClient {
    pub auth_key: String,
    websocket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

const DEFAULT_WS_HOST: &str = "wss://socket.polygon.io";

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Env(#[from] VarError),
    #[error(transparent)]
    Url(#[from] ParseError),
    #[error(transparent)]
    Ws(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("connection closed")]
    Closed,
}

impl WebSocketClient {
    /// Returns a new WebSocket client.
    ///
    /// The `cluster` parameter can be one of `STOCKS_CLUSTER`, `FOREX_CLUSTER`,
    /// or `CRYPTO_CLUSTER`.
    ///
    /// The `auth_key` parameter optionally provides the API key to use for
    /// authentication. If `None` is provided, then the API key specified in the
    /// `POLYGON_AUTH_KEY` environment variable is used.
    ///
    /// # Panics
    ///
    /// This function will panic if `auth_key` is `None` and the
    /// `POLYGON_AUTH_KEY` environment variable is not set.
    pub async fn new(cluster: &str, auth_key: Option<&str>) -> Result<Self, Error> {
        let auth_key_actual = match auth_key {
            Some(v) => String::from(v),
            _ => env::var("POLYGON_AUTH_KEY")?,
        };

        let url_str = format!("{}/{}", DEFAULT_WS_HOST, cluster);
        let url = Url::parse(&url_str)?;
        let websocket = tokio_tungstenite::connect_async(url).await?.0;

        let mut wsc = WebSocketClient {
            auth_key: auth_key_actual,
            websocket,
        };

        wsc.authenticate().await?;

        Ok(wsc)
    }

    async fn authenticate(&mut self) -> Result<(), Error> {
        let msg = format!("{{\"action\":\"auth\",\"params\":\"{}\"}}", self.auth_key);
        self.websocket.send(Message::Text(msg)).await?;
        Ok(())
    }

    /// Subscribes to one or more ticker.
    pub async fn subscribe(&mut self, params: &[&str]) -> Result<(), Error> {
        let msg = format!(
            "{{\"action\":\"subscribe\",\"params\":\"{}\"}}",
            params.join(",")
        );
        self.websocket.send(Message::Text(msg)).await?;
        Ok(())
    }

    /// Unscribes from one or more ticker.
    pub async fn unsubscribe(&mut self, params: &[&str]) -> Result<(), Error> {
        let msg = format!(
            "{{\"action\":\"unsubscribe\",\"params\":\"{}\"}}",
            params.join(",")
        );
        self.websocket.send(Message::Text(msg)).await?;
        Ok(())
    }

    /// Receives a single message.
    pub async fn receive(&mut self) -> Result<Message, Error> {
        Ok(self.websocket.next().await.ok_or(Error::Closed)??)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Clone, Deserialize, Debug)]
    struct ConnectedMessage {
        ev: String,
        status: String,
        #[allow(dead_code)]
        message: String,
    }

    #[tokio::test]
    async fn test_subscribe() {
        let mut socket = WebSocketClient::new(STOCKS_CLUSTER, None).await.unwrap();
        let params = vec!["T.MSFT"];
        socket.subscribe(&params).await.unwrap();
    }

    #[tokio::test]
    async fn test_receive() {
        let mut socket = WebSocketClient::new(STOCKS_CLUSTER, None).await.unwrap();
        let msg = socket.receive().await.unwrap();
        let msg_str = msg.into_text().unwrap();
        let messages: Vec<ConnectedMessage> = serde_json::from_str(&msg_str).unwrap();
        let connected = messages.first().unwrap();
        assert_eq!(connected.ev, "status");
        assert_eq!(connected.status, "connected");
    }
}
