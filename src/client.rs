use std::env;
use std::collections::HashMap;

use crate::types::*;

static DEFAULT_API_URL: &str = "https://api.polygon.io";

pub struct Client {
    pub auth_key: String,
    pub api_url: String,
    pub timeout: Option<u32>,
    client: reqwest::Client,
}

impl Client {
    pub fn new(auth_key: Option<&str>, timeout: Option<u32>) -> Self {
        let api_url = match env::var("POLYGON_API_URL") {
            Ok(v) => v,
            _ => String::from(DEFAULT_API_URL),
        };

        let auth_key_actual = match auth_key {
            Some(v) => String::from(v),
            _ => match env::var("POLYGON_AUTH_KEY") {
                Ok(v ) => String::from(v),
                _ => panic!("POLYGON_AUTH_KEY not set")
            }
        };

        Client {
            auth_key: auth_key_actual,
            api_url: api_url,
            timeout: timeout,
            client: reqwest::Client::new(),
        }
    }

    async fn send_request<RespType>(self, uri: &str, query_params: &HashMap<&str, &str>) -> Result<RespType, reqwest::Error> 
        where RespType : serde::de::DeserializeOwned
    {
        let res = self.client
            .get(format!("{}{}", self.api_url, uri))
            .bearer_auth(self.auth_key)
            .query(query_params)
            .send()
            .await?;

        res.json::<RespType>().await
    }

    pub async fn reference_tickers(self, query_params: &HashMap<&str, &str>) -> Result<ReferenceTickersResponse, reqwest::Error> {
        self.send_request::<ReferenceTickersResponse>("/v3/reference/tickers", query_params).await
    }

    pub async fn reference_ticker_types(self, query_params: &HashMap<&str, &str>) -> Result<ReferenceTickerTypesResponse, reqwest::Error> {
        self.send_request::<ReferenceTickerTypesResponse>("/v2/reference/types", query_params).await
    }
    
    pub async fn reference_ticker_details(self, stocks_ticker: &str, query_params: &HashMap<&str, &str>) -> Result<ReferenceTickerDetailsResponse, reqwest::Error> {
        let uri = format!("/v1/meta/symbols/{}/company", stocks_ticker);
        self.send_request::<ReferenceTickerDetailsResponse>( &uri, query_params).await
    }
    
    pub async fn reference_ticker_details_vx(self, stocks_ticker: &str, query_params: &HashMap<&str, &str>) -> Result<ReferenceTickerDetailsResponseVX, reqwest::Error> {
        let uri = format!("/vX/reference/tickers/{}", stocks_ticker);
        self.send_request::<ReferenceTickerDetailsResponseVX>( &uri, query_params).await
    }
    
    pub async fn reference_ticker_news(self, query_params: &HashMap<&str, &str>) -> Result<ReferenceTickerNewsResponse, reqwest::Error> {
        self.send_request::<ReferenceTickerNewsResponse>("/v2/reference/news", query_params).await
    }
    
    pub async fn reference_markets(self, query_params: &HashMap<&str, &str>) -> Result<ReferenceMarketsResponse, reqwest::Error> {
        self.send_request::<ReferenceMarketsResponse>("/v2/reference/markets", query_params).await
    }
    
    pub async fn reference_locales(self, query_params: &HashMap<&str, &str>) -> Result<ReferenceLocalesResponse, reqwest::Error> {
        self.send_request::<ReferenceLocalesResponse>("/v2/reference/locales", query_params).await
    }
    
    pub async fn reference_stock_splits(self, stocks_ticker: &str, query_params: &HashMap<&str, &str>) -> Result<ReferenceStockSplitsResponse, reqwest::Error> {
        let uri = format!("/v2/reference/splits/{}", stocks_ticker);
        self.send_request::<ReferenceStockSplitsResponse>( &uri, query_params).await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::client::Client;

    #[test]
    fn test_reference_tickers() {
        let mut query_params = HashMap::new();
        query_params.insert("ticker", "MSFT");
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_tickers(&query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.count, 1);
        assert_eq!(resp.results[0].market, "stocks");
        assert_eq!(resp.results[0].currency_name, "usd");
    }
    
    #[test]
    fn test_reference_ticker_types() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_ticker_types(&query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.results.types["CS"], "Common Stock");
        assert_eq!(resp.results.index_types["INDEX"], "Index");
    }

    #[test]
    fn test_reference_ticker_details() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_ticker_details("MSFT", &query_params)
        ).unwrap();
        assert_eq!(resp.country, "usa");
        assert_eq!(resp.name, "Microsoft Corporation");
        assert_eq!(resp.symbol, "MSFT");
    }

    #[test]
    fn test_reference_ticker_details_vx() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_ticker_details_vx("MSFT", &query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.results.ticker, "MSFT");
        assert_eq!(resp.results.currency_name, "usd");
    }
    
    #[test]
    fn test_reference_ticker_news() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_ticker_news(&query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
    } 

    #[test]
    fn test_reference_markets() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_markets(&query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
        let bond = resp.results.iter().find(|x| x.market == "BONDS");
        assert_eq!(bond.is_some(), true);
        assert_eq!(bond.unwrap().desc, "Bonds");
    }

    #[test]
    fn test_reference_locales() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_locales(&query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
        let bond = resp.results.iter().find(|x| x.locale == "US");
        assert_eq!(bond.is_some(), true);
        assert_eq!(bond.unwrap().name, "United States of America");
    }

    #[test]
    fn test_reference_stock_splits() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_stock_splits("MSFT", &query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
        let bond = resp.results.iter().find(|x| x.ex_date == "1998-02-23");
        assert_eq!(bond.is_some(), true);
        assert_eq!(bond.unwrap().ratio, 0.5);
    }


}