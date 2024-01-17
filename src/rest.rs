//! REST client for [polygon.io](https://polygon.io).
//!
//! # Authentication
//!
//! Use an [API key](https://polygon.io/dashboard/api-keys) to authenticate.
//! This can be provided through the `auth_key` parameter to
//! [`RESTClient::new()`] or through the `POLYGON_AUTH_KEY` environment variable.
//!
//! # Example
//!
//! ```
//! use std::collections::HashMap;
//!
//! use polygon_client::rest::RESTClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = RESTClient::new(None, None);
//!     let query_params = HashMap::new();
//!     let resp = client.reference_tickers(&query_params)
//!         .await
//!         .expect("failed to query tickers");
//!     for res in resp.results {
//!         println!("ticker: {}", res.ticker);
//!     }
//! }
//! ```
use std::collections::HashMap;
use std::env;

use crate::types::*;

static DEFAULT_API_URL: &str = "https://api.polygon.io";

pub struct RESTClient {
    /// The API key to use for requests.
    pub auth_key: String,
    /// The API URL to use for requests.
    ///
    /// The default API URL is <https://api.polygon.io>.
    pub api_url: String,
    client: reqwest::Client,
}

impl RESTClient {
    /// Returns a new REST client.
    ///
    /// The `auth_key` parameter optionally provides the API key to use for
    /// authentication. If `None` is provided, then the API key specified in the
    /// `POLYGON_AUTH_KEY` environment variable is used.
    ///
    /// The `timeout` parameter optionally provides the duration to wait for a
    /// response to a request.
    ///
    /// # Panics
    ///
    /// This function will panic if `auth_key` is `None` and the
    /// `POLYGON_AUTH_KEY` environment variable is not set.
    pub fn new(auth_key: Option<&str>, timeout: Option<core::time::Duration>) -> Self {
        let api_url = match env::var("POLYGON_API_URL") {
            Ok(v) => v,
            _ => String::from(DEFAULT_API_URL),
        };

        let auth_key_actual = match auth_key {
            Some(v) => String::from(v),
            _ => match env::var("POLYGON_AUTH_KEY") {
                Ok(v) => v,
                _ => panic!("POLYGON_AUTH_KEY not set"),
            },
        };

        let mut client = reqwest::ClientBuilder::new();

        if let Some(timeout) = timeout {
            client = client.timeout(timeout);
        }

        RESTClient {
            auth_key: auth_key_actual,
            api_url,
            client: client.build().unwrap(),
        }
    }

    async fn send_request<RespType>(
        &self,
        uri: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<RespType, reqwest::Error>
    where
        RespType: serde::de::DeserializeOwned,
    {
        let res = self
            .client
            .get(format!("{}{}", self.api_url, uri))
            .bearer_auth(&self.auth_key)
            .query(query_params)
            .send()
            .await;

        match res {
            Ok(res) => {
                if res.status() == 200 {
                    res.json::<RespType>().await
                } else {
                    Err(res.error_for_status().err().unwrap())
                }
            }
            Err(e) => Err(e),
        }
    }

    //
    // Reference APIs
    //

    /// Query all ticker symbols supported by polygon.io using the
    /// [/v3/reference/tickers](https://polygon.io/docs/get_v3_reference_tickers_anchor)
    /// API.
    pub async fn reference_tickers(
        &self,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceTickersResponse, reqwest::Error> {
        self.send_request::<ReferenceTickersResponse>("/v3/reference/tickers", query_params)
            .await
    }

    /// Get a mapping of ticker types to their descriptive names using the
    /// [/v2/reference/types](https://polygon.io/docs/get_v2_reference_types_anchor)
    /// API.
    pub async fn reference_ticker_types(
        &self,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceTickerTypesResponse, reqwest::Error> {
        self.send_request::<ReferenceTickerTypesResponse>("/v2/reference/types", query_params)
            .await
    }

    /// Get details for a ticker symbol's company/entity using the
    /// [/v1/meta/symbols/{stocks_ticker}/company](https://polygon.io/docs/get_v1_meta_symbols__stocksTicker__company_anchor)
    /// API.
    pub async fn reference_ticker_details(
        &self,
        stocks_ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceTickerDetailsResponse, reqwest::Error> {
        let uri = format!("/v1/meta/symbols/{}/company", stocks_ticker);
        self.send_request::<ReferenceTickerDetailsResponse>(&uri, query_params)
            .await
    }

    /// Get details for a ticker symbol's company/entity using the
    /// [/vX/reference/tickers/{stocks_ticker}](https://polygon.io/docs/get_vX_reference_tickers__ticker__anchor)
    /// API.
    pub async fn reference_ticker_details_vx(
        &self,
        stocks_ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceTickerDetailsResponseVX, reqwest::Error> {
        let uri = format!("/vX/reference/tickers/{}", stocks_ticker);
        self.send_request::<ReferenceTickerDetailsResponseVX>(&uri, query_params)
            .await
    }

    /// Get the most recent news articles related to a stock ticker symbol using
    /// the [/v2/reference/news](https://polygon.io/docs/get_v2_reference_news_anchor) API.
    pub async fn reference_ticker_news(
        &self,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceTickerNewsResponse, reqwest::Error> {
        self.send_request::<ReferenceTickerNewsResponse>("/v2/reference/news", query_params)
            .await
    }

    /// Get a list of markets that are currently supported by polygon.io using
    /// the [/v2/reference/markets](https://polygon.io/docs/get_v2_reference_markets_anchor) API.
    pub async fn reference_markets(
        &self,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceMarketsResponse, reqwest::Error> {
        self.send_request::<ReferenceMarketsResponse>("/v2/reference/markets", query_params)
            .await
    }

    /// Get a list of locales currently supported by polygon.io using the
    /// [/v2/reference/locales](https://polygon.io/docs/get_v2_reference_locales_anchor) API.
    pub async fn reference_locales(
        &self,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceLocalesResponse, reqwest::Error> {
        self.send_request::<ReferenceLocalesResponse>("/v2/reference/locales", query_params)
            .await
    }

    /// Get a list of historical stock splits for a ticker symbol using the
    /// [/v2/reference/splits/{stocks_ticker}](https://polygon.io/docs/get_v2_reference_splits__stocksTicker__anchor) API.
    pub async fn reference_stock_splits(
        &self,
        stocks_ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceStockSplitsResponse, reqwest::Error> {
        let uri = format!("/v2/reference/splits/{}", stocks_ticker);
        self.send_request::<ReferenceStockSplitsResponse>(&uri, query_params)
            .await
    }

    /// Get a list of historical dividends for a stock using the
    /// [/v3/reference/dividends/{stocks_ticker}](https://polygon.io/docs/get_v3_reference_dividends__stocksTicker__anchor) API.
    pub async fn reference_stock_dividends(
        &self,
        stocks_ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceStockDividendsResponse, reqwest::Error> {
        let uri = format!("/v3/reference/dividends?ticker={}", stocks_ticker);
        self.send_request::<ReferenceStockDividendsResponse>(&uri, query_params)
            .await
    }

    /// Get historical financial data for a stock ticker using the
    /// [/v2/reference/financials/{stocks_ticker}](https://polygon.io/docs/get_v2_reference_financials__stocksTicker__anchor) API.
    pub async fn reference_stock_financials(
        &self,
        stocks_ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceStockFinancialsResponse, reqwest::Error> {
        let uri = format!("/v2/reference/financials/{}", stocks_ticker);
        self.send_request::<ReferenceStockFinancialsResponse>(&uri, query_params)
            .await
    }

    /// Get historical financial data for a stock ticker using the
    /// [/vX/reference/financials](https://polygon.io/docs/get_vX_reference_financials_anchor) API.
    pub async fn reference_stock_financials_vx(
        &self,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceStockFinancialsVXResponse, reqwest::Error> {
        self.send_request::<ReferenceStockFinancialsVXResponse>(
            "/vX/reference/financials",
            query_params,
        )
        .await
    }

    /// Get upcoming market holidays and their open/close items using the
    /// [/v1/marketstatus/upcoming](https://polygon.io/docs/get_v1_marketstatus_upcoming_anchor) API.
    pub async fn reference_market_holidays(
        &self,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceMarketStatusUpcomingResponse, reqwest::Error> {
        self.send_request::<ReferenceMarketStatusUpcomingResponse>(
            "/v1/marketstatus/upcoming",
            query_params,
        )
        .await
    }

    /// Get the current trading status of the exchanges and overall financial
    /// markets using the [/v1/marketstatus/now](https://polygon.io/docs/get_v1_marketstatus_now_anchor) API.
    pub async fn reference_market_status(
        &self,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ReferenceMarketStatusNowResponse, reqwest::Error> {
        self.send_request::<ReferenceMarketStatusNowResponse>("/v1/marketstatus/now", query_params)
            .await
    }

    //
    // Stock equities APIs
    //

    /// Get a list of stock exchanges which are supported by polygon.io using
    /// the [/v1/meta/exchanges](https://polygon.io/docs/get_v1_meta_exchanges_anchor) API.
    pub async fn stock_equities_exchanges(
        &self,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesExchangesResponse, reqwest::Error> {
        self.send_request::<StockEquitiesExchangesResponse>("/v1/meta/exchanges", query_params)
            .await
    }

    /// Get a unified numerical mapping for conditions on trades and quotes
    /// using the [/v1/meta/conditions/{tick_type}](https://polygon.io/docs/get_v1_meta_conditions__ticktype__anchor) API.
    pub async fn stock_equities_condition_mappings(
        &self,
        tick_type: TickType,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesConditionMappingsResponse, reqwest::Error> {
        let uri = format!(
            "/v1/meta/conditions/{}",
            tick_type.to_string().to_lowercase()
        );
        self.send_request::<StockEquitiesConditionMappingsResponse>(&uri, query_params)
            .await
    }

    /// Get the most recent trade for a given stock using the
    /// [/v2/last/trade/{stocks_ticker}](https://polygon.io/docs/get_v2_last_trade__stocksTicker__anchor) API.
    pub async fn stock_equities_historic_trades(
        &self,
        stocks_ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesHistoricTradesResponse, reqwest::Error> {
        let uri = format!("/v2/last/trade/{}", stocks_ticker);
        self.send_request::<StockEquitiesHistoricTradesResponse>(&uri, query_params)
            .await
    }

    /// Get the most recent NBBO quote tick for a given stock using the
    /// [/v2/last/nbbo/{stocks_ticker}](https://polygon.io/docs/get_v2_last_nbbo__stocksTicker__anchor) API.
    pub async fn stock_equities_last_quote_for_a_symbol(
        &self,
        stocks_ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesLastQuoteForASymbolResponse, reqwest::Error> {
        let uri = format!("/v2/last/nbbo/{}", stocks_ticker);
        self.send_request::<StockEquitiesLastQuoteForASymbolResponse>(&uri, query_params)
            .await
    }

    /// Get the open, close, and afterhours prices of a stock symbol on a
    /// certain date using the [/v1/open-close/{stocks_ticker}/{date}](https://polygon.io/docs/get_v1_open-close__stocksTicker___date__anchor) API.
    pub async fn stock_equities_daily_open_close(
        &self,
        stocks_ticker: &str,
        date: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesDailyOpenCloseResponse, reqwest::Error> {
        let uri = format!("/v1/open-close/{}/{}", stocks_ticker, date);
        self.send_request::<StockEquitiesDailyOpenCloseResponse>(&uri, query_params)
            .await
    }

    /// Get aggregate bars for a stock over a given date range in custom time
    /// window sizes using the [/v2/aggs/ticker/{stocks_ticker}/range/{multiplier}/{timespan}/{from}/{to}](https://polygon.io/docs/get_v2_aggs_ticker__stocksTicker__range__multiplier___timespan___from___to__anchor) API.
    pub async fn stock_equities_aggregates(
        &self,
        stocks_ticker: &str,
        multiplier: u32,
        timespan: &str,
        from: &str,
        to: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesAggregatesResponse, reqwest::Error> {
        let uri = format!(
            "/v2/aggs/ticker/{}/range/{}/{}/{}/{}",
            stocks_ticker, multiplier, timespan, from, to
        );
        self.send_request::<StockEquitiesAggregatesResponse>(&uri, query_params)
            .await
    }

    /// Get the daily open, high, low, and close for the entire stocks and
    /// equities market using the [/v2/aggs/grouped/locale/{locale}/market/{market}/{date}](https://polygon.io/docs/get_v2_aggs_grouped_locale_us_market_stocks__date__anchor) API.
    pub async fn stock_equities_grouped_daily(
        &self,
        locale: &str,
        market: &str,
        date: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesGroupedDailyResponse, reqwest::Error> {
        let uri = format!(
            "/v2/aggs/grouped/locale/{}/market/{}/{}",
            locale, market, date
        );
        self.send_request::<StockEquitiesGroupedDailyResponse>(&uri, query_params)
            .await
    }

    /// Get the previous day's open, high, low, and close for the specified
    /// stock ticker using the [/v2/aggs/ticker/{stocks_ticker}/prev](https://polygon.io/docs/get_v2_aggs_ticker__stocksTicker__prev_anchor) API.
    pub async fn stock_equities_previous_close(
        &self,
        stocks_ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesPreviousCloseResponse, reqwest::Error> {
        let uri = format!("/v2/aggs/ticker/{}/prev", stocks_ticker);
        self.send_request::<StockEquitiesPreviousCloseResponse>(&uri, query_params)
            .await
    }

    /// Get the current minute, day, and previous day's aggregate, as well as
    /// the last trade and quote for all traded stock symbols using the [/v2/snapshot/locale/{locale}/markets/{market}/tickers](https://polygon.io/docs/get_v2_snapshot_locale_us_markets_stocks_tickers_anchor) API.
    pub async fn stock_equities_snapshot_all_tickers(
        &self,
        locale: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesSnapshotAllTickersResponse, reqwest::Error> {
        let uri = format!("/v2/snapshot/locale/{}/markets/stocks/tickers", locale);
        self.send_request::<StockEquitiesSnapshotAllTickersResponse>(&uri, query_params)
            .await
    }

    /// Get the current minute, day, and previous day's aggregate, as well as
    /// the last trade and quote for a single traded stock ticker using the [/v2/snapshot/locale/{locale}/markets/stocks/tickers/{ticker}](https://polygon.io/docs/get_v2_snapshot_locale_us_markets_stocks_tickers__stocksTicker__anchor) API.
    pub async fn stock_equities_snapshot_single_ticker(
        &self,
        locale: &str,
        ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesSnapshotAllTickersResponse, reqwest::Error> {
        let uri = format!(
            "/v2/snapshot/locale/{}/markets/stocks/tickers/{}",
            locale, ticker
        );
        self.send_request::<StockEquitiesSnapshotAllTickersResponse>(&uri, query_params)
            .await
    }

    /// Get the current top 20 gainers or losers of the day in the
    /// stocks/equities markets using the [/v2/snapshot/locale/{locale}/markets/stocks/{direction}](https://polygon.io/docs/get_v2_snapshot_locale_us_markets_stocks__direction__anchor) API.
    pub async fn stock_equities_snapshot_gainers_losers(
        &self,
        locale: &str,
        direction: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<StockEquitiesSnapshotGainersLosersResponse, reqwest::Error> {
        let uri = format!(
            "/v2/snapshot/locale/{}/markets/stocks/{}",
            locale, direction
        );
        self.send_request::<StockEquitiesSnapshotGainersLosersResponse>(&uri, query_params)
            .await
    }

    //
    // Forex APIs
    //

    /// Get aggregate bars for a forex pair over a given date range in custom
    /// time window sizes using the [/v2/aggs/ticker/{forexTicker}/range/{multiplier}/{timespan}/{from}/{to}](https://polygon.io/docs/get_v2_aggs_ticker__forexTicker__range__multiplier___timespan___from___to__anchor) API.
    pub async fn forex_currencies_aggregates(
        &self,
        forex_ticker: &str,
        multiplier: u32,
        timespan: &str,
        from: &str,
        to: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ForexCurrenciesAggregatesResponse, reqwest::Error> {
        let uri = format!(
            "/v2/aggs/ticker/{}/range/{}/{}/{}/{}",
            forex_ticker, multiplier, timespan, from, to
        );
        self.send_request::<ForexCurrenciesAggregatesResponse>(&uri, query_params)
            .await
    }

    /// Get the daily open, high, low, and close for the entire forex markets
    /// using the [/v2/aggs/grouped/locale/global/market/fx/{date}](https://polygon.io/docs/get_v2_aggs_grouped_locale_global_market_fx__date__anchor) API.
    pub async fn forex_currencies_grouped_daily(
        &self,
        date: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ForexCurrenciesGroupedDailyResponse, reqwest::Error> {
        let uri = format!("/v2/aggs/grouped/locale/global/market/fx/{}", date);
        self.send_request::<ForexCurrenciesGroupedDailyResponse>(&uri, query_params)
            .await
    }

    /// Get the previous day's open, high, low, and close for the specified
    /// forex pair using the [/v2/aggs/ticker/{forex_ticker}/prev](https://polygon.io/docs/get_v2_aggs_ticker__forexTicker__prev_anchor) API.
    pub async fn forex_currencies_previous_close(
        &self,
        forex_ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<ForexCurrenciesPreviousCloseResponse, reqwest::Error> {
        let uri = format!("/v2/aggs/ticker/{}/prev", forex_ticker);
        self.send_request::<ForexCurrenciesPreviousCloseResponse>(&uri, query_params)
            .await
    }

    //
    // Crypto APIs
    //

    /// Get a list of cryptocurrency exchanges which are supported by polygon.io
    /// using the [/v1/meta/crypto-exchanges](https://polygon.io/docs/get_v1_meta_crypto-exchanges_anchor) API.
    pub async fn crypto_crypto_exchanges(
        &self,
        query_params: &HashMap<&str, &str>,
    ) -> Result<CryptoCryptoExchangesResponse, reqwest::Error> {
        self.send_request::<CryptoCryptoExchangesResponse>(
            "/v1/meta/crypto-exchanges",
            query_params,
        )
        .await
    }

    /// Get the open and close prices of a cryptocurrency symbol on a certain day
    /// using [/v1/open-close/crypto/{from}/{to}/{date}](https://polygon.io/docs/get_v1_open-close_crypto__from___to___date__anchor) API.
    pub async fn crypto_daily_open_close(
        &self,
        from: &str,
        to: &str,
        date: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<CryptoDailyOpenCloseResponse, reqwest::Error> {
        let uri = format!("/v1/open-close/crypto/{}/{}/{}", from, to, date);
        self.send_request::<CryptoDailyOpenCloseResponse>(&uri, query_params)
            .await
    }

    /// Get aggregate bars for a cryptocurrency over a given date range in custom
    /// time window sizes using the [/v2/aggs/ticker/{cryptoTicker}/range/{multiplier}/{timespan}/{from}/{to}](https://polygon.io/docs/get_v2_aggs_ticker__cryptoTicker__range__multiplier___timespan___from___to__anchor) API.
    pub async fn crypto_aggregates(
        &self,
        crypto_ticker: &str,
        multiplier: u32,
        timespan: &str,
        from: &str,
        to: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<CryptoAggregatesResponse, reqwest::Error> {
        let uri = format!(
            "/v2/aggs/ticker/{}/range/{}/{}/{}/{}",
            crypto_ticker, multiplier, timespan, from, to
        );
        self.send_request::<CryptoAggregatesResponse>(&uri, query_params)
            .await
    }

    /// Get the daily open, high, low, and close for the entire crypto markets
    /// using the [/v2/aggs/grouped/locale/global/market/crypto/{date}](https://polygon.io/docs/get_v2_aggs_grouped_locale_global_market_crypto__date__anchor) API.
    pub async fn crypto_grouped_daily(
        &self,
        date: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<CryptoGroupedDailyResponse, reqwest::Error> {
        let uri = format!("/v2/aggs/grouped/locale/global/market/crypto/{}", date);
        self.send_request::<CryptoGroupedDailyResponse>(&uri, query_params)
            .await
    }

    /// Get the previous day's open, high, low, and close for the specified
    /// cryptocurrency using the [/v2/aggs/ticker/{crypto_ticker}/prev](https://polygon.io/docs/get_v2_aggs_ticker__cryptoTicker__prev_anchor) API.
    pub async fn crypto_previous_close(
        &self,
        crypto_ticker: &str,
        query_params: &HashMap<&str, &str>,
    ) -> Result<CryptoPreviousCloseResponse, reqwest::Error> {
        let uri = format!("/v2/aggs/ticker/{}/prev", crypto_ticker);
        self.send_request::<CryptoPreviousCloseResponse>(&uri, query_params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::rest::RESTClient;
    use crate::types::*;
    use std::collections::HashMap;

    #[test]
    fn test_reference_tickers() {
        let mut query_params = HashMap::new();
        query_params.insert("ticker", "MSFT");
        let resp =
            tokio_test::block_on(RESTClient::new(None, None).reference_tickers(&query_params))
                .unwrap();
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.count, 1);
        assert_eq!(resp.results[0].market, "stocks");
        assert_eq!(resp.results[0].currency_name, "usd");
    }

    #[test]
    fn test_reference_ticker_types() {
        let query_params = HashMap::new();
        let resp =
            tokio_test::block_on(RESTClient::new(None, None).reference_ticker_types(&query_params))
                .unwrap();
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.results.types["CS"], "Common Stock");
        assert_eq!(resp.results.index_types["INDEX"], "Index");
    }

    #[test]
    fn test_reference_ticker_details() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).reference_ticker_details("MSFT", &query_params),
        )
        .unwrap();
        assert_eq!(resp.country, "usa");
        assert_eq!(resp.name, "Microsoft Corporation");
        assert_eq!(resp.symbol, "MSFT");
    }

    #[test]
    fn test_reference_ticker_details_vx() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).reference_ticker_details_vx("MSFT", &query_params),
        )
        .unwrap();
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.results.ticker, "MSFT");
        assert_eq!(resp.results.currency_name, "usd");
    }

    #[test]
    fn test_reference_ticker_news() {
        let query_params = HashMap::new();
        let resp =
            tokio_test::block_on(RESTClient::new(None, None).reference_ticker_news(&query_params))
                .unwrap();
        assert_eq!(resp.status, "OK");
    }

    #[test]
    fn test_reference_markets() {
        let query_params = HashMap::new();
        let resp =
            tokio_test::block_on(RESTClient::new(None, None).reference_markets(&query_params))
                .unwrap();
        assert_eq!(resp.status, "OK");
        let bond = resp.results.iter().find(|x| x.market == "BONDS");
        assert!(bond.is_some());
        assert_eq!(bond.unwrap().desc, "Bonds");
    }

    #[test]
    fn test_reference_locales() {
        let query_params = HashMap::new();
        let resp =
            tokio_test::block_on(RESTClient::new(None, None).reference_locales(&query_params))
                .unwrap();
        assert_eq!(resp.status, "OK");
        let bond = resp.results.iter().find(|x| x.locale == "US");
        assert!(bond.is_some());
        assert_eq!(bond.unwrap().name, "United States of America");
    }

    #[test]
    fn test_reference_stock_splits() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).reference_stock_splits("MSFT", &query_params),
        )
        .unwrap();
        assert_eq!(resp.status, "OK");
        let bond = resp.results.iter().find(|x| x.ex_date == "1998-02-23");
        assert!(bond.is_some());
        assert_eq!(bond.unwrap().ratio, 0.5);
    }

    #[test]
    fn test_reference_stock_dividends() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).reference_stock_dividends("MSFT", &query_params),
        )
        .unwrap();
        assert_eq!(resp.status, "OK");
        let bond = resp.results.iter().find(|x| x.ex_date == "2021-02-17");
        assert!(bond.is_some());
        assert_eq!(bond.unwrap().amount, 0.56);
    }

    #[test]
    fn test_reference_stock_financials() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).reference_stock_financials("MSFT", &query_params),
        )
        .unwrap();
        assert_eq!(resp.status, "OK");
        let fin = resp.results.iter().find(|x| x.ticker == "MSFT");
        assert!(fin.is_some());
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).reference_stock_financials("AAPL", &query_params),
        )
        .unwrap();
        let fin = resp.results.iter().find(|x| x.ticker == "AAPL");
        assert!(fin.is_some());
    }

    #[test]
    fn test_reference_stock_financials_vx() {
        let mut query_params = HashMap::new();
        query_params.insert("ticker", "MSFT");
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).reference_stock_financials_vx(&query_params),
        )
        .unwrap();
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.count, 1);
        let result = resp.results.first().unwrap();
        for v in &result.financials.balance_sheet {
            println!("{} = true", v.0);
        }
        let income_statement = &result.financials.income_statement;
        assert!(income_statement.contains_key(FAC_REVENUES));
        assert!(income_statement.get(FAC_REVENUES).unwrap().unit.is_some());
        assert_eq!(
            income_statement
                .get(FAC_REVENUES)
                .unwrap()
                .unit
                .as_ref()
                .unwrap(),
            "USD"
        );
    }

    #[test]
    fn test_reference_market_holidays() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).reference_market_holidays(&query_params),
        )
        .unwrap();
        assert_ne!(resp.len(), 0);
    }

    #[test]
    fn test_reference_market_status() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).reference_market_status(&query_params),
        )
        .unwrap();
        assert_ne!(resp.exchanges.len(), 0);
    }

    #[test]
    fn test_stock_equities_exchanges() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).stock_equities_exchanges(&query_params),
        )
        .unwrap();
        assert_ne!(resp.len(), 0);
        let dji = resp
            .iter()
            .find(|x| x.code.is_some() && x.code.as_ref().unwrap() == "DJI");
        assert!(dji.is_some());
        assert_eq!(dji.unwrap().market, "index");
    }

    #[test]
    fn test_stock_equities_condition_mappings() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None)
                .stock_equities_condition_mappings(TickType::Trades, &query_params),
        )
        .unwrap();
        assert_ne!(resp.len(), 0);
        let regular = resp.iter().find(|x| x.1 == "Regular");
        assert!(regular.is_some());
    }

    #[test]
    fn test_stock_equities_historic_trades() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).stock_equities_historic_trades("MSFT", &query_params),
        )
        .unwrap();
        assert_eq!(resp.results.T.unwrap(), "MSFT");
    }

    #[test]
    fn test_stock_equities_last_quote_for_a_symbol() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None)
                .stock_equities_last_quote_for_a_symbol("MSFT", &query_params),
        )
        .unwrap();
        assert_eq!(resp.results.T.unwrap(), "MSFT");
    }

    #[test]
    fn test_stock_equities_daily_open_close() {
        let query_params = HashMap::new();
        let resp =
            tokio_test::block_on(RESTClient::new(None, None).stock_equities_daily_open_close(
                "MSFT",
                "2020-10-14",
                &query_params,
            ))
            .unwrap();
        assert_eq!(resp.symbol, "MSFT");
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.open, 223f64);
        assert_eq!(resp.high, 224.22);
        assert_eq!(resp.low, 219.13);
        assert_eq!(resp.close, 220.86);
        assert_eq!(resp.volume, 23451713f64);
        assert_eq!(resp.after_hours, 220.3);
        assert_eq!(resp.pre_market, 224.03);
    }

    #[test]
    fn test_stock_equities_aggregates() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(RESTClient::new(None, None).stock_equities_aggregates(
            "MSFT",
            1,
            "day",
            "2020-10-14",
            "2020-10-14",
            &query_params,
        ))
        .unwrap();
        assert_eq!(resp.ticker, "MSFT");
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.query_count, 1);
        assert_eq!(resp.results_count, 1);
        let result = resp.results.first().unwrap();
        assert_eq!(result.v, 23451713f64);
        assert_eq!(result.vw.unwrap(), 221.41);
        assert_eq!(result.o, 223f64);
        assert_eq!(result.c, 220.86);
        assert_eq!(result.h, 224.22);
        assert_eq!(result.l, 219.13);
        assert_eq!(result.t.unwrap(), 1602648000000);
        assert_eq!(result.n.unwrap(), 244243f64);
    }

    #[test]
    fn test_stock_equities_grouped_daily() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(RESTClient::new(None, None).stock_equities_grouped_daily(
            "us",
            "stocks",
            "2020-10-14",
            &query_params,
        ))
        .unwrap();
        assert_eq!(resp.status, "OK");
        let msft = resp
            .results
            .iter()
            .find(|x| x.T.is_some() && x.T.as_ref().unwrap() == "MSFT");
        assert!(msft.is_some());
        assert!(msft.unwrap().vw.is_some());
        assert_eq!(msft.unwrap().vw.unwrap(), 221.41);
        assert_eq!(msft.unwrap().o, 223f64);
        assert_eq!(msft.unwrap().h, 224.22);
        assert_eq!(msft.unwrap().l, 219.13);
    }

    #[test]
    fn test_stock_equities_previous_close() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).stock_equities_previous_close("MSFT", &query_params),
        )
        .unwrap();
        assert_eq!(resp.ticker, "MSFT");
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.results_count, 1);
        let result = resp.results.first();
        assert!(result.is_some());
        assert!(result.unwrap().T.is_some());
        assert_eq!(result.unwrap().T.as_ref().unwrap(), "MSFT");
    }

    #[test]
    fn test_stock_equities_snapshot_all_tickers() {
        let query_params = HashMap::new();
        let _resp = tokio_test::block_on(
            RESTClient::new(None, None).stock_equities_snapshot_all_tickers("us", &query_params),
        )
        .unwrap();
    }

    #[test]
    fn test_stock_equities_snapshot_gainers_losers() {
        let query_params = HashMap::new();
        let _resp = tokio_test::block_on(
            RESTClient::new(None, None).stock_equities_snapshot_gainers_losers(
                "us",
                "gainers",
                &query_params,
            ),
        )
        .unwrap();
    }

    #[test]
    fn test_crypto_crypto_exchanges() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).crypto_crypto_exchanges(&query_params),
        )
        .unwrap();
        assert_ne!(resp.len(), 0);
        let coinbase = resp.iter().find(|x| x.name == "Coinbase");
        assert!(coinbase.is_some());
    }

    #[test]
    fn test_forex_currencies_aggregates() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(RESTClient::new(None, None).forex_currencies_aggregates(
            "C:EURUSD",
            1,
            "day",
            "2020-10-14",
            "2020-10-14",
            &query_params,
        ))
        .unwrap();
        assert_eq!(resp.ticker, "C:EURUSD");
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.query_count, 1);
        assert_eq!(resp.results_count, 1);
        let result = resp.results.first().unwrap();
        assert_eq!(result.v, 211796f64);
        assert_eq!(result.vw.unwrap(), 1.1748);
        assert_eq!(result.o, 1.17439);
        assert_eq!(result.c, 1.17496);
        assert_eq!(result.h, 1.1771);
        assert_eq!(result.l, 1.17198);
        assert_eq!(result.t.unwrap(), 1602633600000);
        assert_eq!(result.n.unwrap(), 211796f64);
    }

    #[test]
    fn test_forex_currencies_grouped_daily() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).forex_currencies_grouped_daily("2020-10-14", &query_params),
        )
        .unwrap();
        assert_eq!(resp.status, "OK");
        let msft = resp
            .results
            .iter()
            .find(|x| x.T.is_some() && x.T.as_ref().unwrap() == "C:EURMUR");
        assert!(msft.is_some());
        assert!(msft.unwrap().vw.is_some());
        assert_eq!(msft.unwrap().vw.unwrap(), 45.2081);
        assert_eq!(msft.unwrap().o, 45.37);
        assert_eq!(msft.unwrap().h, 45.59);
        assert_eq!(msft.unwrap().l, 44.83);
    }

    #[test]
    fn test_forex_currencies_previous_close() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).forex_currencies_previous_close("C:EURUSD", &query_params),
        )
        .unwrap();
        assert_eq!(resp.ticker, "C:EURUSD");
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.results_count, 1);
        let result = resp.results.first();
        assert!(result.is_some());
        assert!(result.unwrap().T.is_some());
        assert_eq!(result.unwrap().T.as_ref().unwrap(), "C:EURUSD");
    }

    #[test]
    fn test_crypto_daily_open_close() {
        let mut query_params = HashMap::new();
        query_params.insert("adjusted", "true");
        let resp = tokio_test::block_on(RESTClient::new(None, None).crypto_daily_open_close(
            "BTC",
            "USD",
            "2020-10-14",
            &query_params,
        ))
        .unwrap();
        assert_eq!(resp.symbol, "BTC-USD");
        assert!(resp.is_utc);
        assert_eq!(resp.open, 11443f64);
        assert_eq!(resp.close, 11427.7);
    }

    #[test]
    fn test_crypto_aggregates() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(RESTClient::new(None, None).crypto_aggregates(
            "X:BTCUSD",
            1,
            "day",
            "2020-10-14",
            "2020-10-14",
            &query_params,
        ))
        .unwrap();
        assert_eq!(resp.ticker, "X:BTCUSD");
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.query_count, 1);
        assert_eq!(resp.results_count, 1);
        let result = resp.results.first().unwrap();
        assert_eq!(result.vw.unwrap(), 11405.5019);
        assert_eq!(result.o, 11443f64);
        assert_eq!(result.c, 11427.7);
        assert_eq!(result.h, 11564f64);
        assert_eq!(result.l, 11284.27);
        assert_eq!(result.t.unwrap(), 1602633600000);
        assert_eq!(result.n.unwrap(), 142439f64);
    }

    #[test]
    fn test_crypto_grouped_daily() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).crypto_grouped_daily("2020-10-14", &query_params),
        )
        .unwrap();
        assert_eq!(resp.status, "OK");
        let msft = resp
            .results
            .iter()
            .find(|x| x.T.is_some() && x.T.as_ref().unwrap() == "X:LTCUSD");
        assert!(msft.is_some());
        assert!(msft.unwrap().vw.is_some());
        assert_eq!(msft.unwrap().vw.unwrap(), 50.1376);
        assert_eq!(msft.unwrap().o, 49.981);
        assert_eq!(msft.unwrap().h, 51.095);
        assert_eq!(msft.unwrap().l, 49.2427);
    }

    #[test]
    fn test_crypto_previous_close() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            RESTClient::new(None, None).crypto_previous_close("X:BTCUSD", &query_params),
        )
        .unwrap();
        assert_eq!(resp.ticker, "X:BTCUSD");
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.results_count, 1);
        let result = resp.results.first();
        assert!(result.is_some());
        assert!(result.unwrap().T.is_some());
        assert_eq!(result.unwrap().T.as_ref().unwrap(), "X:BTCUSD");
    }
}
