use serde;
use serde::Deserialize;

use std::collections::HashMap;

//
// v3/reference/tickers
//

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickersResponseTickerV3 {
    pub ticker: String,
    pub name: String,
    pub market: String,
    pub locale: String,
    pub primary_exchange: String,
    #[serde(rename = "type")]
    pub ticker_type: String,
    pub active: bool,
    pub currency_name: String,
    pub cik: String,
    pub composite_figi: String,
    pub share_class_figi: String,
    pub last_updated_utc: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickersResponseV3 {
    pub results: Vec<ReferenceTickersResponseTickerV3>,
    pub status: String,
    pub request_id: String,
    pub count: u32,
    pub next_url: Option<String>
}

pub type ReferenceTickersResponse = ReferenceTickersResponseV3;

//
// v2/reference/types
//

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickerTypesResultsV2 {
    pub types: HashMap<String, String>,
    #[serde(rename = "indexTypes")]
    pub index_types: HashMap<String, String>
}
 
#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickerTypesResponseV2 {
    pub status: String,
    pub results: ReferenceTickerTypesResultsV2,
}

pub type ReferenceTickerTypesResponse = ReferenceTickerTypesResponseV2;

//
// v1/meta/symbols/{stocksTicker}/company
//

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickerDetailsResponseV1 {
    pub logo: String,
    pub exchange: String,
    #[serde(rename = "exchangeSymbol")]
    pub exchange_symbol: String,
    #[serde(rename = "type")]
    pub ticker_type: String,
    pub name: String,
    pub symbol: String,
    pub listdate: String,
    pub cik: String,
    pub bloomberg: String,
    pub fiji: Option<String>,
    pub sic: u32,
    pub country: String,
    pub industry: String,
    pub sector: String,
    pub marketcap: u64,
    pub employees: u64,
    pub phone: String,
    pub ceo: String,
    pub url: String,
    pub description: String,
    pub hq_address: String,
    pub hq_country: String,
    pub similar: Vec<String>,
    pub tags: Vec<String>,
    pub updated: String,
    pub active: bool
}

pub type ReferenceTickerDetailsResponse = ReferenceTickerDetailsResponseV1;

//
// vX/reference/tickers/{ticker}
//

#[derive(Clone, Deserialize, Debug)]
pub struct Address {
    pub address1: String,
    pub city: String,
    pub state: String
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickerDetailsResultsVX {
    pub ticker: String,
    pub name: String,
    pub market: String,
    pub locale: String,
    pub primary_exchange: String,
    #[serde(rename = "type")]
    pub ticker_type: String,
    pub active: bool,
    pub currency_name: String,
    pub cik: String,
    pub composite_fiji: Option<String>,
    pub share_class_fiji: Option<String>,
    pub last_updated_utc: String,
    pub delisted_utc: Option<String>,
    pub outstanding_shares: f64,
    pub market_cap: f64,
    pub phone_number: String,
    pub address: Address,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickerDetailsResponseVX {
    pub results: ReferenceTickerDetailsResultsVX,
    pub status: String,
    pub request_id: String,
    pub count: u32
}

//
// v2/reference/news
//

#[derive(Clone, Deserialize, Debug)]
pub struct Publisher {
    pub name: String,
    pub homepage_url: String,
    pub logo_url: String,
    pub favicon_url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickerNewsResultsV2 {
    pub id: String,
    pub publisher: Publisher,
    pub title: String,
    pub author: String,
    pub published_utc: String,
    pub article_url: String,
    pub tickers: Vec<String>,
    pub amp_url: String,
    pub image_url: String,
    pub description: String,
    pub keywords: Option<Vec<String>>,
}


#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickerNewsResponseV2 {
    pub results: Vec<ReferenceTickerNewsResultsV2>,
    pub status: String,
    pub request_id: String,
    pub count: u32,
    pub next_url: Option<String>
}

pub type ReferenceTickerNewsResponse = ReferenceTickerNewsResponseV2;

//
// v2/reference/markets
//

#[derive(Clone, Deserialize, Debug)]
pub struct Market {
    pub market: String,
    pub desc: String
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceMarketsResponseV2 {
    pub status: String,
    pub results: Vec<Market>
}

pub type ReferenceMarketsResponse  = ReferenceMarketsResponseV2;

//
// v2/reference/locales
//

#[derive(Clone, Deserialize, Debug)]
pub struct Locale {
    pub locale: String,
    pub name: String
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceLocalesResponseV2 {
    pub status: String,
    pub results: Vec<Locale>,
}

pub type ReferenceLocalesResponse = ReferenceLocalesResponseV2;

//
// v2/reference/splits/{stockTicker}
//

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockSplitsResultV2 {
    pub ticker: String,
    #[serde(rename = "exDate")]
    pub ex_date: String,
    #[serde(rename = "paymentDate")]
    pub payment_date: String,
    #[serde(rename = "declaredDate")]
    pub declared_date: Option<String>,
    pub ratio: f64,
    pub tofactor: Option<u32>,
    pub forfactor: Option<u32>
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockSplitsResponseV2 {
    pub status: String,
    pub count: u32,
    pub results: Vec<ReferenceStockSplitsResultV2>,
}

pub type ReferenceStockSplitsResponse = ReferenceStockSplitsResponseV2;