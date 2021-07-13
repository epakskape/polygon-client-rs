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
    pub tickers: Option<Vec<String>>,
    pub amp_url: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
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

//
// v2/reference/dividends/{stocksTicker}
//

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockDividendsResultV2 {
    pub ticker: String,
    #[serde(rename = "exDate")]
    pub ex_date: String,
    #[serde(rename = "paymentDate")]
    pub payment_date: String,
    #[serde(rename = "recordDate")]
    pub record_date: String,
    pub amount: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockDividendsResponseV2 {
    pub status: String,
    pub count: u32,
    pub results: Vec<ReferenceStockDividendsResultV2>,
}

pub type ReferenceStockDividendsResponse = ReferenceStockDividendsResponseV2;

//
// v2/reference/financials/{stocksTicker}
//

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockFinancialsResultV2 {
    pub ticker: String,
    pub period: String,
    #[serde(rename = "calendarDate")]
    pub calendar_date: String,
    #[serde(rename = "reportPeriod")]
    pub report_period: String,
    pub updated: String,
    #[serde(rename = "accumulatedOtherComprehensiveIncome")]
    pub accumulated_other_comprehensive_income: Option<i64>,
    pub assets: Option<i64>,
    #[serde(rename = "assetsAverage")]
    pub assets_average: Option<i64>,
    #[serde(rename = "assetsCurrent")]
    pub assets_current: Option<i64>,
    #[serde(rename = "assetTurnover")]
    pub asset_turnover: Option<f64>,
    #[serde(rename = "assetsNonCurrent")]
    pub assets_non_current: Option<i64>,
    #[serde(rename = "bookValuePerShare")]
    pub book_value_per_share: Option<f64>,
    #[serde(rename = "capitalExpenditure")]
    pub capital_expenditure: Option<i64>,
    #[serde(rename = "cashAndEquivalents")]
    pub cash_and_equivalents: Option<i64>,
    #[serde(rename = "cashAndEquivalentsUSD")]
    pub cash_and_equivalents_usd: Option<i64>,
    #[serde(rename = "costOfRevenue")]
    pub cost_of_revenue: Option<i64>,
    #[serde(rename = "consolidatedIncome")]
    pub consolidated_income: Option<i64>,
    #[serde(rename = "currentRatio")]
    pub current_ratio: Option<f64>,
    #[serde(rename = "debtToEquityRatio")]
    pub debt_to_equity_ratio: Option<f64>,
    pub debt: Option<u64>,
    #[serde(rename = "debtCurrent")]
    pub debt_current: Option<u64>,
    #[serde(rename = "debtNonCurrent")]
    pub debt_non_current: Option<u64>,
    #[serde(rename = "debtUSD")]
    pub debt_usd: Option<u64>,
    #[serde(rename = "deferredRevenue")]
    pub deferred_revenue: Option<u64>,
    #[serde(rename = "depreciationAmortizationAndAccretion")]
    pub depreciation_amortization_and_accretion: Option<i64>,
    pub deposits: Option<u64>,
    #[serde(rename = "dividendYield")]
    pub dividend_yield: Option<f64>,
    #[serde(rename = "dividendsPerBasicCommonShare")]
    pub dividends_per_basic_common_share: Option<f64>,
    #[serde(rename = "earningBeforeInterestTaxes")]
    pub earning_before_interest_taxes: Option<i64>,
    #[serde(rename = "earningBeforeInterestTaxesUSD")]
    pub earning_before_interest_taxes_usd: Option<i64>,
    #[serde(rename = "earningsBeforeInterestTaxesDepreciationAmortization")]
    pub earnings_before_interest_taxes_drepreciation_amortization: Option<i64>,
    #[serde(rename = "earningsBeforeInterestTaxesDepreciationAmortizationUSD")]
    pub earnings_before_interest_taxes_drepreciation_amortization_usd: Option<i64>,
    #[serde(rename = "earningsBeforeTax")]
    pub earnings_before_tax: Option<i64>,
    #[serde(rename = "earningsPerBasicShare")]
    pub earnings_per_basic_share: Option<f64>,
    #[serde(rename = "earningsPerBasicShareUSD")]
    pub earnings_per_basic_share_usd: Option<f64>,
    #[serde(rename = "earningsPerDilutedShare")]
    pub earnings_per_diluted_share: Option<f64>,
    #[serde(rename = "EBITDAMargin")]
    pub ebitda_margin: Option<f64>,
    #[serde(rename = "shareholdersEquity")]
    pub shareholders_equity: Option<i64>,
    #[serde(rename = "shareholdersEquityUSD")]
    pub shareholders_equity_usd: Option<i64>,
    #[serde(rename = "enterpriseValue")]
    pub enterprise_value: Option<i64>,
    #[serde(rename = "enterpriseValueOverEBIT")]
    pub enterprise_value_over_ebit: Option<i64>,
    #[serde(rename = "enterpriseValueOverEBITDA")]
    pub enterprise_value_over_ebitda: Option<f64>,
    #[serde(rename = "freeCashFlow")]
    pub free_cash_flow: Option<i64>,
    #[serde(rename = "freeCashFlowPerShare")]
    pub free_cash_flow_per_share: Option<f64>,
    #[serde(rename = "foreignCurrencyUSDExchangeRate")]
    pub foreign_currency_usd_exchange_rate: Option<f64>,
    #[serde(rename = "grossProfit")]
    pub gross_profit: Option<i64>,
    #[serde(rename = "grossMargin")]
    pub gross_margin: Option<f64>,
    #[serde(rename = "goodwillAndIntangibleAssets")]
    pub goodwill_and_intangible_assets: Option<i64>,
    #[serde(rename = "interestExpense")]
    pub interest_expense: Option<i64>,
    #[serde(rename = "investedCapital")]
    pub invested_capital: Option<i64>,
    pub inventory: Option<i64>,
    pub investments: Option<i64>,
    #[serde(rename = "investmentsCurrent")]
    pub investments_current: Option<i64>,
    #[serde(rename = "investmentsNonCurrent")]
    pub investments_non_current: Option<i64>,
    #[serde(rename = "totalLiabilities")]
    pub total_liabilities: Option<i64>,
    #[serde(rename = "currentLiabilities")]
    pub current_liabilities: Option<i64>,
    #[serde(rename = "liabilitiesNonCurrent")]
    pub liabilities_non_current: Option<i64>,
    #[serde(rename = "marketCapitalization")]
    pub market_capitalization: Option<i64>,
    #[serde(rename = "netCashFlow")]
    pub net_cash_flow: Option<i64>,
    #[serde(rename = "netCashFlowBusinessAcquisitionsDisposals")]
    pub net_cash_flow_business_acquisitions_disposals: Option<i64>,
    #[serde(rename = "issuanceEquityShares")]
    pub issuance_equity_shares: Option<i64>,
    #[serde(rename = "issuanceDebtSecurities")]
    pub issuance_debt_securities: Option<i64>,
    #[serde(rename = "paymentDividendsOtherCashDistributions")]
    pub payment_dividends_other_cash_distributions: Option<i64>,
    #[serde(rename = "netCashFlowFromFinancing")]
    pub net_cash_flow_from_financing: Option<i64>,
    #[serde(rename = "netCashFlowFromInvesting")]
    pub net_cash_flow_from_investing: Option<i64>,
    #[serde(rename = "netCashFlowInvestmentAcquisitionsDisposals")]
    pub net_cash_flow_investment_acquisitions_disposals: Option<i64>,
    #[serde(rename = "netCashFlowFromOperations")]
    pub net_cash_flow_from_operations: Option<i64>,
    #[serde(rename = "effectOfExchangeRateChangesOnCash")]
    pub effect_of_exchange_rate_changes_on_cash: Option<i64>,
    #[serde(rename = "netIncome")]
    pub net_income: Option<i64>,
    #[serde(rename = "netIncomeCommonStock")]
    pub net_income_common_stock: Option<i64>,
    #[serde(rename = "netIncomeCommonStockUSD")]
    pub net_income_common_stock_usd: Option<i64>,
    #[serde(rename = "netLossIncomeFromDiscontinuedOperations")]
    pub net_loss_income_from_discontinued_operations: Option<i64>,
    #[serde(rename = "netIncomeToNonControllingInterests")]
    pub net_income_to_non_controlling_interests: Option<i64>,
    #[serde(rename = "profitMargin")]
    pub profit_margin: Option<f64>,
    #[serde(rename = "operatingExpenses")]
    pub operating_expenses: Option<i64>,
    #[serde(rename = "operatingIncome")]
    pub operating_income: Option<i64>,
    #[serde(rename = "tradeAndNonTradePayables")]
    pub trade_and_non_trade_payables: Option<i64>,
    #[serde(rename = "payoutRatio")]
    pub payout_ratio: Option<f64>,
    #[serde(rename = "priceToBookValue")]
    pub price_to_book_value: Option<f64>,
    #[serde(rename = "priceEarnings")]
    pub price_earnings: Option<f64>,
    #[serde(rename = "priceToEarningsRatio")]
    pub price_to_earnings_ratio: Option<f64>,
    #[serde(rename = "propertyPlantEquipmentNet")]
    pub property_plant_equipement_net: Option<i64>,
    #[serde(rename = "preferredDividendsIncomeStatementImpact")]
    pub preferred_dividends_income_statement_impact: Option<i64>,
    #[serde(rename = "sharePriceAdjustedClose")]
    pub share_price_adjusted_close: Option<f64>,
    #[serde(rename = "priceSales")]
    pub price_sales: Option<f64>,
    #[serde(rename = "priceToSalesRatio")]
    pub price_to_sales_ratio: Option<f64>,
    #[serde(rename = "tradeAndNonTradeReceivables")]
    pub trade_and_non_trade_receivables: Option<i64>,
    #[serde(rename = "accumulatedRetainedEarningsDeficit")]
    pub accumulated_retained_earnings_deficit: Option<i64>,
    pub revenues: Option<i64>,
    #[serde(rename = "revenuesUSD")]
    pub revenues_usd: Option<i64>,
    #[serde(rename = "researchAndDevelopmentExpense")]
    pub research_and_development_expense: Option<i64>,
    #[serde(rename = "returnOnAverageAssets")]
    pub return_on_average_assets: Option<f64>,
    #[serde(rename = "returnOnAverageEquity")]
    pub return_on_average_equity: Option<f64>,
    #[serde(rename = "returnOnInvestedCapital")]
    pub return_on_invested_capital: Option<f64>,
    #[serde(rename = "returnOnSales")]
    pub return_on_sales: Option<f64>,
    #[serde(rename = "shareBasedCompensation")]
    pub share_based_compensation: Option<i64>,
    #[serde(rename = "sellingGeneralAndAdministrativeExpense")]
    pub selling_general_and_administrative_expense: Option<i64>,
    #[serde(rename = "shareFactor")]
    pub share_factor: Option<f64>,
    pub shares: Option<u64>,
    #[serde(rename = "weightedAverageShares")]
    pub weighted_average_shares: Option<i64>,
    #[serde(rename = "weightedAverageSharesDiluted")]
    pub weighted_average_shares_diluted: Option<i64>,
    #[serde(rename = "salesPerShare")]
    pub sales_per_share: Option<f64>,
    #[serde(rename = "tangibleAssetValue")]
    pub tangible_asset_value: Option<i64>,
    #[serde(rename = "taxAssets")]
    pub tax_assets: Option<i64>,
    #[serde(rename = "incomeTaxExpense")]
    pub income_tax_expense: Option<i64>,
    #[serde(rename = "taxLiabilities")]
    pub tax_liabilities: Option<i64>,
    #[serde(rename = "tangibleAssetsBookValuePerShare")]
    pub tangible_assets_book_value_per_share: Option<f64>,
    #[serde(rename = "workingCapital")]
    pub working_capital: Option<i64>
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockFinancialsResponseV2 {
    pub status: String,
    pub results: Vec<ReferenceStockFinancialsResultV2>,
}

pub type ReferenceStockFinancialsResponse = ReferenceStockFinancialsResponseV2;

//
// v1/marketstatus/upcoming
//

#[derive(Clone, Deserialize, Debug)]
pub struct MarketStatusUpcoming {
    pub exchange: String,
    pub name: String,
    pub date: String,
    pub status: String,
    pub open: Option<String>,
    pub close: Option<String>
}

pub type ReferenceMarketStatusUpcomingResponse = Vec<MarketStatusUpcoming>;

//
// v1/marketstatus/now
//

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceMarketStatusNowResponseV1 {
    pub market: String,
    #[serde(rename = "earlyHours")]
    pub early_hours: bool,
    #[serde(rename = "afterHours")]
    pub after_hours: bool,
    #[serde(rename = "serverTime")]
    pub server_time: String,
    pub exchanges: HashMap<String, String>,
    pub currencies: HashMap<String, String>
}

pub type ReferenceMarketStatusNowResponse = ReferenceMarketStatusNowResponseV1;

//
// v1/meta/exchanges
//

#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesExchangeV1 {
    pub id: u64,
    #[serde(rename = "type")]
    pub exchange_type: String,
    pub market: String,
    pub mic: Option<String>,
    pub name: String,
    pub tape: Option<String>,
    pub code: Option<String>,
}

pub type StockEquitiesExchangesResponse = Vec<StockEquitiesExchangeV1>;