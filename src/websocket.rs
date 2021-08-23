//! WebSocket client for <https://polygon.io>.

use std::env;
use url::Url;

use tungstenite::{Message, WebSocket};
use tungstenite::client::connect;

pub const STOCKS_CLUSTER: &str = "stocks";
pub const FOREX_CLUSTER: &str = "forex";
pub const CRYPTO_CLUSTER: &str = "crypto";

pub struct WebSocketClient {
    pub auth_key: String,
    websocket: WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>
}

static DEFAULT_WS_HOST: &str = "wss://socket.polygon.io";

impl WebSocketClient {
    pub fn new(cluster: &str, auth_key: Option<&str>) -> Self {
        let auth_key_actual = match auth_key {
            Some(v) => String::from(v),
            _ => match env::var("POLYGON_AUTH_KEY") {
                Ok(v) => String::from(v),
                _ => panic!("POLYGON_AUTH_KEY not set"),
            },
        };

        let url_str = format!("{}/{}", DEFAULT_WS_HOST, cluster);
        let url = Url::parse(&url_str).unwrap();
        let sock = connect(url)
            .expect("failed to connect")
            .0;

        let mut wsc = WebSocketClient { 
            auth_key: auth_key_actual,
            websocket: sock
        };

        wsc._authenticate();

        wsc
    }

    fn _authenticate(&mut self) {
        let str = format!("{{\"action\":\"auth\",\"params\":\"{}\"}}", self.auth_key);
        self.websocket.write_message(Message::Text(str.into())).expect("failed to authenticate");
    }

    pub fn subscribe(&mut self, params: &Vec<&str>) {
        let str = params.join(",");
        let msg = format!("{{\"action\":\"subscribe\",\"params\":\"{}\"}}", &str);
        self.websocket.write_message(Message::Text(msg.into())).expect("failed to subscribe");
    }
    
    pub fn unsubscribe(&mut self, params: &Vec<&str>) {
        let str = params.join(",");
        let msg = format!("{{\"action\":\"unsubscribe\",\"params\":\"{}\"}}", &str);
        self.websocket.write_message(Message::Text(msg.into())).expect("failed to unsubscribe");
    }
}

#[cfg(test)]
mod tests {
    use crate::websocket::WebSocketClient;
    use crate::websocket::STOCKS_CLUSTER;

    #[test]
    fn test_subscribe() {
        let mut socket = WebSocketClient::new(STOCKS_CLUSTER, None);
        let params = vec! ["T.MSFT"];
        socket.subscribe(&params);
    }
}