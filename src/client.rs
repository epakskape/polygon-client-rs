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

    // 
    // Reference APIs
    //

    pub async fn reference_tickers(self, query_params: &HashMap<&str, &str>) -> Result<ReferenceTickersResponse, reqwest::Error> {
        self.send_request::<ReferenceTickersResponse>("/v3/reference/tickers", query_params).await
    }

    pub async fn reference_ticker_types(self, query_params: &HashMap<&str, &str>) -> Result<ReferenceTickerTypesResponse, reqwest::Error> {
        self.send_request::<ReferenceTickerTypesResponse>("/v2/reference/types", query_params).await
    }
    
    pub async fn reference_ticker_details(self, stocks_ticker: &str, query_params: &HashMap<&str, &str>) -> Result<ReferenceTickerDetailsResponse, reqwest::Error> {
        let uri = format!("/v1/meta/symbols/{}/company", stocks_ticker);
        self.send_request::<ReferenceTickerDetailsResponse>(&uri, query_params).await
    }
    
    pub async fn reference_ticker_details_vx(self, stocks_ticker: &str, query_params: &HashMap<&str, &str>) -> Result<ReferenceTickerDetailsResponseVX, reqwest::Error> {
        let uri = format!("/vX/reference/tickers/{}", stocks_ticker);
        self.send_request::<ReferenceTickerDetailsResponseVX>(&uri, query_params).await
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
        self.send_request::<ReferenceStockSplitsResponse>(&uri, query_params).await
    }
    
    pub async fn reference_stock_dividends(self, stocks_ticker: &str, query_params: &HashMap<&str, &str>) -> Result<ReferenceStockDividendsResponse, reqwest::Error> {
        let uri = format!("/v2/reference/dividends/{}", stocks_ticker);
        self.send_request::<ReferenceStockDividendsResponse>(&uri, query_params).await
    }
    
    pub async fn reference_stock_financials(self, stocks_ticker: &str, query_params: &HashMap<&str, &str>) -> Result<ReferenceStockFinancialsResponse, reqwest::Error> {
        let uri = format!("/v2/reference/financials/{}", stocks_ticker);
        self.send_request::<ReferenceStockFinancialsResponse>(&uri, query_params).await
    }
    
    pub async fn reference_stock_financials_vx(self, query_params: &HashMap<&str, &str>) -> Result<ReferenceStockFinancialsVXResponse, reqwest::Error> {
        self.send_request::<ReferenceStockFinancialsVXResponse>("/vX/reference/financials", query_params).await
    }
    
    pub async fn reference_market_holidays(self, query_params: &HashMap<&str, &str>) -> Result<ReferenceMarketStatusUpcomingResponse, reqwest::Error> {
        self.send_request::<ReferenceMarketStatusUpcomingResponse>("/v1/marketstatus/upcoming", query_params).await
    }
    
    pub async fn reference_market_status(self, query_params: &HashMap<&str, &str>) -> Result<ReferenceMarketStatusNowResponse, reqwest::Error> {
        self.send_request::<ReferenceMarketStatusNowResponse>("/v1/marketstatus/now", query_params).await
    }

    //
    // Stock equities APIs
    //
    
    pub async fn stock_equities_exchanges(self, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesExchangesResponse, reqwest::Error> {
        self.send_request::<StockEquitiesExchangesResponse>("/v1/meta/exchanges", query_params).await
    }
    
    pub async fn stock_equities_condition_mappings(self, tick_type: TickType, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesConditionMappingsResponse, reqwest::Error> {
        let uri = format!("/v1/meta/conditions/{}", tick_type.to_string().to_lowercase());
        self.send_request::<StockEquitiesConditionMappingsResponse>(&uri, query_params).await
    }
 
    pub async fn stock_equities_historic_trades(self, stocks_ticker: &str, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesHistoricTradesResponse, reqwest::Error> {
        let uri = format!("/v2/last/trade/{}", stocks_ticker);
        self.send_request::<StockEquitiesHistoricTradesResponse>(&uri, query_params).await
    }

    pub async fn stock_equities_last_quote_for_a_symbol(self, stocks_ticker: &str, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesLastQuoteForASymbolResponse, reqwest::Error> {
        let uri = format!("/v2/last/nbbo/{}", stocks_ticker);
        self.send_request::<StockEquitiesLastQuoteForASymbolResponse>(&uri, query_params).await
    }

    pub async fn stock_equities_daily_open_close(self, stocks_ticker: &str, date: &str, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesDailyOpenCloseResponse, reqwest::Error> {
        let uri = format!("/v1/open-close/{}/{}", stocks_ticker, date);
        self.send_request::<StockEquitiesDailyOpenCloseResponse>(&uri, query_params).await
    }

    pub async fn stock_equities_aggregates(self, stocks_ticker: &str, multiplier: u32, timespan: &str, from: &str, to: &str, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesAggregatesResponse, reqwest::Error> {
        let uri = format!("/v2/aggs/ticker/{}/range/{}/{}/{}/{}", stocks_ticker, multiplier, timespan, from, to);
        self.send_request::<StockEquitiesAggregatesResponse>(&uri, query_params).await
    }

    pub async fn stock_equities_grouped_daily(self, locale: &str, market: &str, date: &str, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesGroupedDailyResponse, reqwest::Error> {
        let uri = format!("/v2/aggs/grouped/locale/{}/market/{}/{}", locale, market, date);
        self.send_request::<StockEquitiesGroupedDailyResponse>(&uri, query_params).await
    }

    pub async fn stock_equities_previous_close(self, stocks_ticker: &str, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesPreviousCloseResponse, reqwest::Error> {
        let uri = format!("/v2/aggs/ticker/{}/prev", stocks_ticker);
        self.send_request::<StockEquitiesPreviousCloseResponse>(&uri, query_params).await
    }

    pub async fn stock_equities_snapshot_all_tickers(self, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesSnapshotAllTickersResponse, reqwest::Error> {
        let uri = format!("/v2/snapshot/locale/us/markets/stocks/tickers");
        self.send_request::<StockEquitiesSnapshotAllTickersResponse>(&uri, query_params).await
    }

    pub async fn stock_equities_snapshot_single_ticker(self, ticker: &str, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesSnapshotAllTickersResponse, reqwest::Error> {
        let uri = format!("/v2/snapshot/locale/us/markets/stocks/tickers/{}", ticker);
        self.send_request::<StockEquitiesSnapshotAllTickersResponse>(&uri, query_params).await
    }

    pub async fn stock_equities_snapshot_gainers_losers(self, direction: &str, query_params: &HashMap<&str, &str>) -> Result<StockEquitiesSnapshotGainersLosersResponse, reqwest::Error> {
        let uri = format!("/v2/snapshot/locale/us/markets/stocks/{}", direction);
        self.send_request::<StockEquitiesSnapshotGainersLosersResponse>(&uri, query_params).await
    }

    //
    // Crypto APIs
    //

    pub async fn crypto_crypto_exchanges(self, query_params: &HashMap<&str, &str>) -> Result<CryptoCryptoExchangesResponse, reqwest::Error> {
        self.send_request::<CryptoCryptoExchangesResponse>("/v1/meta/crypto-exchanges", query_params).await
    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::client::Client;
    use crate::types::*;

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

    #[test]
    fn test_reference_stock_dividends() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_stock_dividends("MSFT", &query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
        let bond = resp.results.iter().find(|x| x.ex_date == "2021-02-17");
        assert_eq!(bond.is_some(), true);
        assert_eq!(bond.unwrap().amount, 0.56);
    }

    #[test]
    fn test_reference_stock_financials() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_stock_financials("MSFT", &query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
        let fin = resp.results.iter().find(|x| x.ticker == "MSFT");
        assert_eq!(fin.is_some(), true);
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_stock_financials("AAPL", &query_params)
        ).unwrap();
        let fin = resp.results.iter().find(|x| x.ticker == "AAPL");
        assert_eq!(fin.is_some(), true);
    }

    #[test]
    fn test_reference_stock_financials_vx() {
        let mut query_params = HashMap::new();
        query_params.insert("ticker", "MSFT");
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_stock_financials_vx(&query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.count, 1);
        let result = resp.results.first().unwrap();
        for v in &result.financials.balance_sheet {
            println!("{} = true", v.0);
        }
        let income_statement = &result.financials.income_statement;
        assert_eq!(income_statement.contains_key(FAC_REVENUES), true);
        assert_eq!(income_statement.get(FAC_REVENUES).unwrap().unit.is_some(), true);
        assert_eq!(income_statement.get(FAC_REVENUES).unwrap().unit.as_ref().unwrap(), "USD");
    }


    #[test]
    fn test_reference_market_holidays() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_market_holidays(&query_params)
        ).unwrap();
        assert_ne!(resp.len(), 0);
    }

    #[test]
    fn test_reference_market_status() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).reference_market_status(&query_params)
        ).unwrap();
        assert_ne!(resp.exchanges.len(), 0);
    }

    #[test]
    fn test_stock_equities_exchanges() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).stock_equities_exchanges(&query_params)
        ).unwrap();
        assert_ne!(resp.len(), 0);
        let dji = resp.iter().find(|x| x.code.is_some() && x.code.as_ref().unwrap() == "DJI");
        assert_eq!(dji.is_some(), true);
        assert_eq!(dji.unwrap().market, "index");
    }

    #[test]
    fn test_stock_equities_condition_mappings() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).stock_equities_condition_mappings(TickType::Trades, &query_params)
        ).unwrap();
        assert_ne!(resp.len(), 0);
        let regular = resp.iter().find(|x| x.1 == "Regular");
        assert_eq!(regular.is_some(), true);
    }

    #[test]
    fn test_stock_equities_historic_trades() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).stock_equities_historic_trades("MSFT", &query_params)
        ).unwrap();
        assert_eq!(resp.results.T, "MSFT");
    }

    #[test]
    fn test_stock_equities_last_quote_for_a_symbol() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).stock_equities_last_quote_for_a_symbol("MSFT", &query_params)
        ).unwrap();
        assert_eq!(resp.results.T, "MSFT");
    }

    #[test]
    fn test_stock_equities_daily_open_close() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).stock_equities_daily_open_close("MSFT", "2020-10-14", &query_params)
        ).unwrap();
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
        let resp = tokio_test::block_on(
            Client::new(None, None).stock_equities_aggregates("MSFT", 1, "day", "2020-10-14", "2020-10-14", &query_params)
        ).unwrap();
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
        assert_eq!(result.t, 1602648000000);
        assert_eq!(result.n.unwrap(), 244243f64);
    }

    #[test]
    fn test_stock_equities_grouped_daily() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).stock_equities_grouped_daily("us","stocks", "2020-10-14", &query_params)
        ).unwrap();
        assert_eq!(resp.status, "OK");
        let msft = resp.results.iter().find(|x| x.T.is_some() && x.T.as_ref().unwrap() == "MSFT");
        assert_eq!(msft.is_some(), true);
        assert_eq!(msft.unwrap().vw.is_some(), true);
        assert_eq!(msft.unwrap().vw.unwrap(), 221.41);
        assert_eq!(msft.unwrap().o, 223f64);
        assert_eq!(msft.unwrap().h, 224.22);
        assert_eq!(msft.unwrap().l, 219.13);
    }

    #[test]
    fn test_stock_equities_previous_close() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).stock_equities_previous_close("MSFT", &query_params)
        ).unwrap();
        assert_eq!(resp.ticker, "MSFT");
        assert_eq!(resp.status, "OK");
        assert_eq!(resp.results_count, 1);
        let result = resp.results.first();
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().T.is_some(), true);
        assert_eq!(result.unwrap().T.as_ref().unwrap(), "MSFT");
    }

    #[test]
    fn test_stock_equities_snapshot_all_tickers() {
        let query_params = HashMap::new();
        let _resp = tokio_test::block_on(
            Client::new(None, None).stock_equities_snapshot_all_tickers(&query_params)
        ).unwrap();
    }

    #[test]
    fn test_stock_equities_snapshot_gainers_losers() {
        let query_params = HashMap::new();
        let _resp = tokio_test::block_on(
            Client::new(None, None).stock_equities_snapshot_gainers_losers("gainers", &query_params)
        ).unwrap();
    }


    #[test]
    fn test_crypto_crypto_exchanges() {
        let query_params = HashMap::new();
        let resp = tokio_test::block_on(
            Client::new(None, None).crypto_crypto_exchanges(&query_params)
        ).unwrap();
        assert_ne!(resp.len(), 0);
        let coinbase = resp.iter().find(|x| x.name == "Coinbase");
        assert_eq!(coinbase.is_some(), true);
    }


}