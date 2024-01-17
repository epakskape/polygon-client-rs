//! Data types associated with the REST interfaces.
use serde;
use serde::Deserialize;

use std::collections::HashMap;
use std::fmt;

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
    pub ticker_type: Option<String>,
    pub active: bool,
    pub currency_name: String,
    pub cik: Option<String>,
    pub composite_figi: Option<String>,
    pub share_class_figi: Option<String>,
    pub last_updated_utc: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickersResponseV3 {
    pub results: Vec<ReferenceTickersResponseTickerV3>,
    pub status: String,
    pub request_id: String,
    pub count: u32,
    pub next_url: Option<String>,
}

pub type ReferenceTickersResponse = ReferenceTickersResponseV3;

//
// v2/reference/types
//

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceTickerTypesResultsV2 {
    pub types: HashMap<String, String>,
    #[serde(rename = "indexTypes")]
    pub index_types: HashMap<String, String>,
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
    pub active: bool,
}

pub type ReferenceTickerDetailsResponse = ReferenceTickerDetailsResponseV1;

//
// vX/reference/tickers/{ticker}
//

#[derive(Clone, Deserialize, Debug)]
pub struct Address {
    pub address1: String,
    pub city: String,
    pub state: String,
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
    pub count: u32,
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
    pub next_url: Option<String>,
}

pub type ReferenceTickerNewsResponse = ReferenceTickerNewsResponseV2;

//
// v2/reference/markets
//

#[derive(Clone, Deserialize, Debug)]
pub struct Market {
    pub market: String,
    pub desc: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceMarketsResponseV2 {
    pub status: String,
    pub results: Vec<Market>,
}

pub type ReferenceMarketsResponse = ReferenceMarketsResponseV2;

//
// v2/reference/locales
//

#[derive(Clone, Deserialize, Debug)]
pub struct Locale {
    pub locale: String,
    pub name: String,
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
    pub forfactor: Option<u32>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockSplitsResponseV2 {
    pub status: String,
    pub count: u32,
    pub results: Vec<ReferenceStockSplitsResultV2>,
}

pub type ReferenceStockSplitsResponse = ReferenceStockSplitsResponseV2;

//
// v3/reference/dividends/
//

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockDividendsResultV3 {
    pub cash_amount: f64,
    pub currency: String,
    pub declaration_date: String,
    pub dividend_type: DividendType,
    pub ex_dividend_date: String,
    pub frequency: u32,
    pub pay_date: String,
    pub record_date: String,
    pub ticker: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockDividendsResponseV3 {
    pub next_url: Option<String>,
    pub results: Vec<ReferenceStockDividendsResultV3>,
    pub status: String,
}

pub type ReferenceStockDividendsResponse = ReferenceStockDividendsResponseV3;

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
    pub working_capital: Option<i64>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockFinancialsResponseV2 {
    pub status: String,
    pub results: Vec<ReferenceStockFinancialsResultV2>,
}

pub type ReferenceStockFinancialsResponse = ReferenceStockFinancialsResponseV2;

//
// vX/reference/financials
//

pub const FAC_ASSETS: &str = "assets";
pub const FAC_BALANCE_SHEET_DATE: &str = "balance_sheet_date";
pub const FAC_BALANCE_SHEET_FORMAT: &str = "balance_sheet_format";
pub const FAC_BENEFITS_COSTS_EXPENSES: &str = "benefits_costs_expenses";
pub const FAC_CAPITALIZATION: &str = "capitalization";
pub const FAC_COMMITMENTS_AND_CONTINGENCIES: &str = "commitments_and_contingencies";
pub const FAC_COMPREHENSIVE_INCOME_LOSS: &str = "comprehensive_income_loss";
pub const FAC_COMPREHENSIVE_INCOME_LOSS_ATTRIBUTABLE_TO_NONCONTROLLING_INTEREST: &str =
    "comprehensive_income_loss_attributable_to_noncontrolling_interest";
pub const FAC_COMPREHENSIVE_INCOME_LOSS_ATTRIBUTABLE_TO_PARENT: &str =
    "comprehensive_income_loss_attributable_to_parent";
pub const FAC_COSTS_AND_EXPENSES: &str = "costs_and_expenses";
pub const FAC_COST_OF_REVENUE: &str = "cost_of_revenue";
pub const FAC_COST_OF_REVENUE_GOODS: &str = "cost_of_revenue_goods";
pub const FAC_COST_OF_REVENUE_SERVICES: &str = "cost_of_revenue_services";
pub const FAC_CURRENT_ASSETS: &str = "current_assets";
pub const FAC_CURRENT_LIABILITIES: &str = "current_liabilities";
pub const FAC_DOCUMENT_TYPE: &str = "document_type";
pub const FAC_ENTITY_CENTRAL_INDEX_KEY: &str = "entity_central_index_key";
pub const FAC_ENTITY_FILER_CATEGORY: &str = "entity_filer_category";
pub const FAC_ENTITY_REGISTRANT_NAME: &str = "entity_registrant_name";
pub const FAC_EQUITY: &str = "equity";
pub const FAC_EQUITY_ATTRIBUTABLE_TO_NONCONTROLLING_INTEREST: &str =
    "equity_attributable_to_noncontrolling_interest";
pub const FAC_EQUITY_ATTRIBUTABLE_TO_PARENT: &str = "equity_attributable_to_parent";
pub const FAC_EXCHANGE_GAINS_LOSSES: &str = "exchange_gains_losses";
pub const FAC_EXTRAORDINARY_ITEMS_OF_INCOME_EXPENSE_NET_OF_TAX: &str =
    "extraordinary_items_of_income_expense_net_of_tax";
pub const FAC_FISCAL_PERIOD_FOCUS: &str = "fiscal_period_focus";
pub const FAC_FISCAL_YEAR_END: &str = "fiscal_year_end";
pub const FAC_FISCAL_YEAR_FOCUS: &str = "fiscal_year_focus";
pub const FAC_FIXED_ASSETS: &str = "fixed_assets";
pub const FAC_GAIN_LOSS_ON_DISPOSITION_STOCK_IN_SUBSIDIARY_OR_EQUITY_METHOD_INVESTEE: &str =
    "gain_loss_on_disposition_stock_in_subsidiary_or_equity_method_investee";
pub const FAC_GAIN_LOSS_ON_SALE_PREVIOUSLY_UNISSUED_STOCK_BY_SUBSIDIARY_OR_EQUITY_INVESTEE_NONOPERATING_INCOME: &str = "gain_loss_on_sale_previously_unissued_stock_by_subsidiary_or_equity_investee_nonoperating_income";
pub const FAC_GAIN_LOSS_ON_SALE_PROPERTIES_NET_TAX: &str = "gain_loss_on_sale_properties_net_tax";
pub const FAC_GROSS_PROFIT: &str = "gross_profit";
pub const FAC_INCOME_LOSS_BEFORE_EQUITY_METHOD_INVESTMENTS: &str =
    "income_loss_before_equity_method_investments";
pub const FAC_INCOME_LOSS_FROM_CONTINUING_OPERATIONS_AFTER_TAX: &str =
    "income_loss_from_continuing_operations_after_tax";
pub const FAC_INCOME_LOSS_FROM_CONTINUING_OPERATIONS_BEFORE_TAX: &str =
    "income_loss_from_continuing_operations_before_tax";
pub const FAC_INCOME_LOSS_FROM_DISCONTINUED_OPERATIONS_NET_OF_TAX: &str =
    "income_loss_from_discontinued_operations_net_of_tax";
pub const FAC_INCOME_LOSS_FROM_DISCONTINUED_OPERATIONS_NET_OF_TAX_ADJUSTMENT_TO_PRIOR_YEAR_GAIN_LOSS_ON_DISPOSAL: &str = "income_loss_from_discontinued_operations_net_of_tax_adjustment_to_prior_year_gain_loss_on_disposal";
pub const FAC_INCOME_LOSS_FROM_DISCONTINUED_OPERATIONS_NET_OF_TAX_DURING_PHASE_OUT: &str =
    "income_loss_from_discontinued_operations_net_of_tax_during_phase_out";
pub const FAC_INCOME_LOSS_FROM_DISCONTINUED_OPERATIONS_NET_OF_TAX_GAIN_LOSS_ON_DISPOSAL: &str =
    "income_loss_from_discontinued_operations_net_of_tax_gain_loss_on_disposal";
pub const FAC_INCOME_LOSS_FROM_DISCONTINUED_OPERATIONS_NET_OF_TAX_PROVISION_FOR_GAIN_LOSS_ON_DISPOSAL: &str = "income_loss_from_discontinued_operations_net_of_tax_provision_for_gain_loss_on_disposal";
pub const FAC_INCOME_LOSS_FROM_EQUITY_METHOD_INVESTMENTS: &str =
    "income_loss_from_equity_method_investments";
pub const FAC_INCOME_STATEMENT_FORMAT: &str = "income_statement_format";
pub const FAC_INCOME_STATEMENT_START_PERIOD_YEAR_TO_DATE: &str =
    "income_statement_start_period_year_to_date";
pub const FAC_INCOME_TAX_EXPENSE_BENEFIT: &str = "income_tax_expense_benefit";
pub const FAC_INCOME_TAX_EXPENSE_BENEFIT_CURRENT: &str = "income_tax_expense_benefit_current";
pub const FAC_INCOME_TAX_EXPENSE_BENEFIT_DEFERRED: &str = "income_tax_expense_benefit_deferred";
pub const FAC_INDIRECT_OPERATING_NONOPERATING_COSTS_EXPENSES: &str =
    "indirect_operating_nonoperating_costs_expenses";
pub const FAC_INTEREST_AND_DEBT_EXPENSE: &str = "interest_and_debt_expense";
pub const FAC_INTEREST_AND_DIVIDEND_INCOME_OPERATING: &str =
    "interest_and_dividend_income_operating";
pub const FAC_INTEREST_EXPENSE: &str = "interest_expense";
pub const FAC_INTEREST_EXPENSE_OPERATING: &str = "interest_expense_operating";
pub const FAC_INTEREST_INCOME_EXPENSE_AFTER_PROVISION_FOR_LOSSES: &str =
    "interest_income_expense_after_provision_for_losses";
pub const FAC_INTEREST_INCOME_EXPENSE_OPERATING_NET: &str = "interest_income_expense_operating_net";
pub const FAC_LIABILITIES: &str = "liabilities";
pub const FAC_LIABILITIES_AND_EQUITY: &str = "liabilities_and_equity";
pub const FAC_LONG_TERM_DEBT: &str = "long_term_debt";
pub const FAC_NET_CASH_FLOW: &str = "net_cash_flow";
pub const FAC_NET_CASH_FLOW_CONTINUING: &str = "net_cash_flow_continuing";
pub const FAC_NET_CASH_FLOW_DISCONTINUED: &str = "net_cash_flow_discontinued";
pub const FAC_NET_CASH_FLOW_FROM_FINANCING_ACTIVITIES: &str =
    "net_cash_flow_from_financing_activities";
pub const FAC_NET_CASH_FLOW_FROM_FINANCING_ACTIVITIES_CONTINUING: &str =
    "net_cash_flow_from_financing_activities_continuing";
pub const FAC_NET_CASH_FLOW_FROM_FINANCING_ACTIVITIES_DISCONTINUED: &str =
    "net_cash_flow_from_financing_activities_discontinued";
pub const FAC_NET_CASH_FLOW_FROM_INVESTING_ACTIVITIES: &str =
    "net_cash_flow_from_investing_activities";
pub const FAC_NET_CASH_FLOW_FROM_INVESTING_ACTIVITIES_CONTINUING: &str =
    "net_cash_flow_from_investing_activities_continuing";
pub const FAC_NET_CASH_FLOW_FROM_INVESTING_ACTIVITIES_DISCONTINUED: &str =
    "net_cash_flow_from_investing_activities_discontinued";
pub const FAC_NET_CASH_FLOW_FROM_OPERATING_ACTIVITIES: &str =
    "net_cash_flow_from_operating_activities";
pub const FAC_NET_CASH_FLOW_FROM_OPERATING_ACTIVITIES_CONTINUING: &str =
    "net_cash_flow_from_operating_activities_continuing";
pub const FAC_NET_CASH_FLOW_FROM_OPERATING_ACTIVITIES_DISCONTINUED: &str =
    "net_cash_flow_from_operating_activities_discontinued";
pub const FAC_NET_INCOME_LOSS: &str = "net_income_loss";
pub const FAC_NET_INCOME_LOSS_ATTRIBUTABLE_TO_NONCONTROLLING_INTEREST: &str =
    "net_income_loss_attributable_to_noncontrolling_interest";
pub const FAC_NET_INCOME_LOSS_ATTRIBUTABLE_TO_NONCONTROLLING_INTEREST_PLUS_PREFERRED_STOCK_DIVIDENDS_AND_OTHER_ADJUSTMENTS: &str = "net_income_loss_attributable_to_noncontrolling_interest_plus_preferred_stock_dividends_and_other_adjustments";
pub const FAC_NET_INCOME_LOSS_ATTRIBUTABLE_TO_NONREDEEMABLE_NONCONTROLLING_INTEREST: &str =
    "net_income_loss_attributable_to_nonredeemable_noncontrolling_interest";
pub const FAC_NET_INCOME_LOSS_ATTRIBUTABLE_TO_PARENT: &str =
    "net_income_loss_attributable_to_parent";
pub const FAC_NET_INCOME_LOSS_ATTRIBUTABLE_TO_REDEEMABLE_NONCONTROLLING_INTEREST: &str =
    "net_income_loss_attributable_to_redeemable_noncontrolling_interest";
pub const FAC_NET_INCOME_LOSS_AVAILABLE_TO_COMMON_STOCKHOLDERS_BASIC: &str =
    "net_income_loss_available_to_common_stockholders_basic";
pub const FAC_NONCURRENT_ASSETS: &str = "noncurrent_assets";
pub const FAC_NONCURRENT_LIABILITIES: &str = "noncurrent_liabilities";
pub const FAC_NONINTEREST_EXPENSE: &str = "noninterest_expense";
pub const FAC_NONINTEREST_INCOME: &str = "noninterest_income";
pub const FAC_NONOPERATING_GAINS_LOSSES: &str = "nonoperating_gains_losses";
pub const FAC_NONOPERATING_INCOME_LOSS: &str = "nonoperating_income_loss";
pub const FAC_NONOPERATING_INCOME_LOSS_PLUS_INTEREST_AND_DEBT_EXPENSE: &str =
    "nonoperating_income_loss_plus_interest_and_debt_expense";
pub const FAC_NONOPERATING_INCOME_PLUS_INTEREST_AND_DEBT_EXPENSE_PLUS_INCOME_FROM_EQUITY_METHOD_INVESTMENTS: &str = "nonoperating_income_plus_interest_and_debt_expense_plus_income_from_equity_method_investments";
pub const FAC_OPERATING_AND_NONOPERATING_COSTS_AND_EXPENSES: &str =
    "operating_and_nonoperating_costs_and_expenses";
pub const FAC_OPERATING_AND_NONOPERATING_REVENUES: &str = "operating_and_nonoperating_revenues";
pub const FAC_OPERATING_EXPENSES: &str = "operating_expenses";
pub const FAC_OPERATING_INCOME_LOSS: &str = "operating_income_loss";
pub const FAC_OTHER_COMPREHENSIVE_INCOME_LOSS: &str = "other_comprehensive_income_loss";
pub const FAC_OTHER_COMPREHENSIVE_INCOME_LOSS_ATTRIBUTABLE_TO_NONCONTROLLING_INTEREST: &str =
    "other_comprehensive_income_loss_attributable_to_noncontrolling_interest";
pub const FAC_OTHER_COMPREHENSIVE_INCOME_LOSS_ATTRIBUTABLE_TO_PARENT: &str =
    "other_comprehensive_income_loss_attributable_to_parent";
pub const FAC_OTHER_NONCURRENT_ASSETS_OF_REGULATED_ENTITY: &str =
    "other_noncurrent_assets_of_regulated_entity";
pub const FAC_OTHER_NONCURRENT_LIABILITIES_OF_REGULATED_ENTITY: &str =
    "other_noncurrent_liabilities_of_regulated_entity";
pub const FAC_OTHER_OPERATING_INCOME_EXPENSES: &str = "other_operating_income_expenses";
pub const FAC_OTHER_THAN_FIXED_NONCURRENT_ASSETS: &str = "other_than_fixed_noncurrent_assets";
pub const FAC_PARTICIPATING_SECURITIES_DISTRIBUTED_AND_UNDISTRIBUTED_EARNINGS_LOSS_BASIC: &str =
    "participating_securities_distributed_and_undistributed_earnings_loss_basic";
pub const FAC_PREFERRED_STOCK_DIVIDENDS_AND_OTHER_ADJUSTMENTS: &str =
    "preferred_stock_dividends_and_other_adjustments";
pub const FAC_PROVISION_FOR_LOAN_LEASE_AND_OTHER_LOSSES: &str =
    "provision_for_loan_lease_and_other_losses";
pub const FAC_PUBLIC_UTILITIES_PROPERTY_PLANT_AND_EQUIPMENT_NET: &str =
    "public_utilities_property_plant_and_equipment_net";
pub const FAC_REDEEMABLE_NONCONTROLLING_INTEREST: &str = "redeemable_noncontrolling_interest";
pub const FAC_REDEEMABLE_NONCONTROLLING_INTEREST_COMMON: &str =
    "redeemable_noncontrolling_interest_common";
pub const FAC_REDEEMABLE_NONCONTROLLING_INTEREST_OTHER: &str =
    "redeemable_noncontrolling_interest_other";
pub const FAC_REDEEMABLE_NONCONTROLLING_INTEREST_PREFERRED: &str =
    "redeemable_noncontrolling_interest_preferred";
pub const FAC_RETURN_ON_ASSETS: &str = "return_on_assets";
pub const FAC_RETURN_ON_EQUITY: &str = "return_on_equity";
pub const FAC_RETURN_ON_SALES: &str = "return_on_sales";
pub const FAC_REVENUES: &str = "revenues";
pub const FAC_REVENUES_EXCLUDING_INTEREST_DIVIDENDS: &str = "revenues_excluding_interest_dividends";
pub const FAC_REVENUES_NET_INTEREST_EXPENSE: &str = "revenues_net_interest_expense";
pub const FAC_TEMPORARY_EQUITY: &str = "temporary_equity";
pub const FAC_TEMPORARY_EQUITY_ATTRIBUTABLE_TO_PARENT: &str =
    "temporary_equity_attributable_to_parent";
pub const FAC_TRADING_SYMBOL: &str = "trading_symbol";
pub const FAC_UNDISTRIBUTED_EARNINGS_LOSS_ALLOCATED_TO_PARTICIPATING_SECURITIES_BASIC: &str =
    "undistributed_earnings_loss_allocated_to_participating_securities_basic";

lazy_static! {
    static ref FAC_LIST: Vec<&'static str> = {
        let v = vec![
            FAC_ASSETS,
            FAC_BALANCE_SHEET_DATE,
            FAC_BALANCE_SHEET_FORMAT,
            FAC_BENEFITS_COSTS_EXPENSES,
            FAC_CAPITALIZATION,
            FAC_COMMITMENTS_AND_CONTINGENCIES,
            FAC_COMPREHENSIVE_INCOME_LOSS,
            FAC_COMPREHENSIVE_INCOME_LOSS_ATTRIBUTABLE_TO_NONCONTROLLING_INTEREST,
            FAC_COMPREHENSIVE_INCOME_LOSS_ATTRIBUTABLE_TO_PARENT,
            FAC_COSTS_AND_EXPENSES,
            FAC_COST_OF_REVENUE,
            FAC_COST_OF_REVENUE_GOODS,
            FAC_COST_OF_REVENUE_SERVICES,
            FAC_CURRENT_ASSETS,
            FAC_CURRENT_LIABILITIES,
            FAC_DOCUMENT_TYPE,
            FAC_ENTITY_CENTRAL_INDEX_KEY,
            FAC_ENTITY_FILER_CATEGORY,
            FAC_ENTITY_REGISTRANT_NAME,
            FAC_EQUITY,
            FAC_EQUITY_ATTRIBUTABLE_TO_NONCONTROLLING_INTEREST,
            FAC_EQUITY_ATTRIBUTABLE_TO_PARENT,
            FAC_EXCHANGE_GAINS_LOSSES,
            FAC_EXTRAORDINARY_ITEMS_OF_INCOME_EXPENSE_NET_OF_TAX,
            FAC_FISCAL_PERIOD_FOCUS,
            FAC_FISCAL_YEAR_END,
            FAC_FIXED_ASSETS,
            FAC_GAIN_LOSS_ON_DISPOSITION_STOCK_IN_SUBSIDIARY_OR_EQUITY_METHOD_INVESTEE,
            FAC_GAIN_LOSS_ON_SALE_PREVIOUSLY_UNISSUED_STOCK_BY_SUBSIDIARY_OR_EQUITY_INVESTEE_NONOPERATING_INCOME,
            FAC_GAIN_LOSS_ON_SALE_PROPERTIES_NET_TAX,
            FAC_GROSS_PROFIT,
            FAC_INCOME_LOSS_BEFORE_EQUITY_METHOD_INVESTMENTS,
            FAC_INCOME_LOSS_FROM_CONTINUING_OPERATIONS_AFTER_TAX,
            FAC_INCOME_LOSS_FROM_CONTINUING_OPERATIONS_BEFORE_TAX,
            FAC_INCOME_LOSS_FROM_DISCONTINUED_OPERATIONS_NET_OF_TAX,
            FAC_INCOME_LOSS_FROM_DISCONTINUED_OPERATIONS_NET_OF_TAX_ADJUSTMENT_TO_PRIOR_YEAR_GAIN_LOSS_ON_DISPOSAL,
            FAC_INCOME_LOSS_FROM_DISCONTINUED_OPERATIONS_NET_OF_TAX_DURING_PHASE_OUT,
            FAC_INCOME_LOSS_FROM_DISCONTINUED_OPERATIONS_NET_OF_TAX_GAIN_LOSS_ON_DISPOSAL,
            FAC_INCOME_LOSS_FROM_DISCONTINUED_OPERATIONS_NET_OF_TAX_PROVISION_FOR_GAIN_LOSS_ON_DISPOSAL,
            FAC_INCOME_LOSS_FROM_EQUITY_METHOD_INVESTMENTS,
            FAC_INCOME_STATEMENT_FORMAT,
            FAC_INCOME_STATEMENT_START_PERIOD_YEAR_TO_DATE,
            FAC_INCOME_TAX_EXPENSE_BENEFIT,
            FAC_INCOME_TAX_EXPENSE_BENEFIT_CURRENT,
            FAC_INCOME_TAX_EXPENSE_BENEFIT_DEFERRED,
            FAC_INDIRECT_OPERATING_NONOPERATING_COSTS_EXPENSES,
            FAC_INTEREST_AND_DEBT_EXPENSE,
            FAC_INTEREST_AND_DIVIDEND_INCOME_OPERATING,
            FAC_INTEREST_EXPENSE,
            FAC_INTEREST_EXPENSE_OPERATING,
            FAC_INTEREST_INCOME_EXPENSE_AFTER_PROVISION_FOR_LOSSES,
            FAC_INTEREST_INCOME_EXPENSE_OPERATING_NET,
            FAC_LIABILITIES,
            FAC_LIABILITIES_AND_EQUITY,
            FAC_LONG_TERM_DEBT,
            FAC_NET_CASH_FLOW,
            FAC_NET_CASH_FLOW_CONTINUING,
            FAC_NET_CASH_FLOW_DISCONTINUED,
            FAC_NET_CASH_FLOW_FROM_FINANCING_ACTIVITIES,
            FAC_NET_CASH_FLOW_FROM_FINANCING_ACTIVITIES_CONTINUING,
            FAC_NET_CASH_FLOW_FROM_FINANCING_ACTIVITIES_DISCONTINUED,
            FAC_NET_CASH_FLOW_FROM_INVESTING_ACTIVITIES,
            FAC_NET_CASH_FLOW_FROM_INVESTING_ACTIVITIES_CONTINUING,
            FAC_NET_CASH_FLOW_FROM_INVESTING_ACTIVITIES_DISCONTINUED,
            FAC_NET_CASH_FLOW_FROM_OPERATING_ACTIVITIES,
            FAC_NET_CASH_FLOW_FROM_OPERATING_ACTIVITIES_CONTINUING,
            FAC_NET_CASH_FLOW_FROM_OPERATING_ACTIVITIES_DISCONTINUED,
            FAC_NET_INCOME_LOSS,
            FAC_NET_INCOME_LOSS_ATTRIBUTABLE_TO_NONCONTROLLING_INTEREST,
            FAC_NET_INCOME_LOSS_ATTRIBUTABLE_TO_NONCONTROLLING_INTEREST_PLUS_PREFERRED_STOCK_DIVIDENDS_AND_OTHER_ADJUSTMENTS,
            FAC_NET_INCOME_LOSS_ATTRIBUTABLE_TO_NONREDEEMABLE_NONCONTROLLING_INTEREST,
            FAC_NET_INCOME_LOSS_ATTRIBUTABLE_TO_PARENT,
            FAC_NET_INCOME_LOSS_ATTRIBUTABLE_TO_REDEEMABLE_NONCONTROLLING_INTEREST,
            FAC_NET_INCOME_LOSS_AVAILABLE_TO_COMMON_STOCKHOLDERS_BASIC,
            FAC_NONCURRENT_ASSETS,
            FAC_NONCURRENT_LIABILITIES,
            FAC_NONINTEREST_EXPENSE,
            FAC_NONINTEREST_INCOME,
            FAC_NONOPERATING_GAINS_LOSSES,
            FAC_NONOPERATING_INCOME_LOSS,
            FAC_NONOPERATING_INCOME_LOSS_PLUS_INTEREST_AND_DEBT_EXPENSE,
            FAC_NONOPERATING_INCOME_PLUS_INTEREST_AND_DEBT_EXPENSE_PLUS_INCOME_FROM_EQUITY_METHOD_INVESTMENTS,
            FAC_OPERATING_AND_NONOPERATING_COSTS_AND_EXPENSES,
            FAC_OPERATING_AND_NONOPERATING_REVENUES,
            FAC_OPERATING_EXPENSES,
            FAC_OPERATING_INCOME_LOSS,
            FAC_OTHER_COMPREHENSIVE_INCOME_LOSS,
            FAC_OTHER_COMPREHENSIVE_INCOME_LOSS_ATTRIBUTABLE_TO_NONCONTROLLING_INTEREST,
            FAC_OTHER_COMPREHENSIVE_INCOME_LOSS_ATTRIBUTABLE_TO_PARENT,
            FAC_OTHER_NONCURRENT_ASSETS_OF_REGULATED_ENTITY,
            FAC_OTHER_NONCURRENT_LIABILITIES_OF_REGULATED_ENTITY,
            FAC_OTHER_OPERATING_INCOME_EXPENSES,
            FAC_OTHER_THAN_FIXED_NONCURRENT_ASSETS,
            FAC_PARTICIPATING_SECURITIES_DISTRIBUTED_AND_UNDISTRIBUTED_EARNINGS_LOSS_BASIC,
            FAC_PREFERRED_STOCK_DIVIDENDS_AND_OTHER_ADJUSTMENTS,
            FAC_PROVISION_FOR_LOAN_LEASE_AND_OTHER_LOSSES,
            FAC_PUBLIC_UTILITIES_PROPERTY_PLANT_AND_EQUIPMENT_NET,
            FAC_REDEEMABLE_NONCONTROLLING_INTEREST,
            FAC_REDEEMABLE_NONCONTROLLING_INTEREST_COMMON,
            FAC_REDEEMABLE_NONCONTROLLING_INTEREST_OTHER,
            FAC_REDEEMABLE_NONCONTROLLING_INTEREST_PREFERRED,
            FAC_RETURN_ON_ASSETS,
            FAC_RETURN_ON_EQUITY,
            FAC_RETURN_ON_SALES,
            FAC_REVENUES,
            FAC_REVENUES_EXCLUDING_INTEREST_DIVIDENDS,
            FAC_REVENUES_NET_INTEREST_EXPENSE,
            FAC_TEMPORARY_EQUITY,
            FAC_TEMPORARY_EQUITY_ATTRIBUTABLE_TO_PARENT,
            FAC_TRADING_SYMBOL,
            FAC_UNDISTRIBUTED_EARNINGS_LOSS_ALLOCATED_TO_PARTICIPATING_SECURITIES_BASIC,
        ];
        v
    };
}

#[derive(Clone, Deserialize, Debug)]
pub struct FundamentalAccountingConcept {
    pub formula: Option<String>,
    pub label: Option<String>,
    pub order: Option<u32>,
    pub unit: Option<String>,
    pub value: Option<f64>,
}
#[derive(Clone, Deserialize, Debug)]
pub struct FinancialDimensions {
    pub balance_sheet: HashMap<String, FundamentalAccountingConcept>,
    pub cash_flow_statement: HashMap<String, FundamentalAccountingConcept>,
    pub comprehensive_income: HashMap<String, FundamentalAccountingConcept>,
    pub income_statement: HashMap<String, FundamentalAccountingConcept>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockFinancialsVXResult {
    pub cik: String,
    pub company_name: String,
    pub end_date: Option<String>,
    pub financials: FinancialDimensions,
    pub fiscal_period: String,
    pub fiscal_year: String,
    pub source_filing_file_url: String,
    pub start_date: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ReferenceStockFinancialsVXResponse {
    pub count: u32,
    pub next_url: String,
    pub request_id: String,
    pub results: Vec<ReferenceStockFinancialsVXResult>,
    pub status: String,
}

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
    pub close: Option<String>,
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
    pub currencies: HashMap<String, String>,
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

//
// v1/meta/conditions/{ticktype}
//

#[derive(Debug)]
pub enum TickType {
    Trades,
    Quotes,
}

#[derive(Clone, Deserialize, Debug)]
pub enum DividendType {
    CD, // Consistent dividends paid on schedule
    SC, // Special cash dividends (not to be expected to be consistenly paid)
    LT, // Long-term capital gain distributions
    ST, // Short-term capital gain distributions
}

impl fmt::Display for DividendType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DividendType::CD => write!(f, "CD"),
            DividendType::SC => write!(f, "SC"),
            DividendType::LT => write!(f, "LT"),
            DividendType::ST => write!(f, "ST"),
        }
    }
}

impl fmt::Display for TickType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type StockEquitiesConditionMappingsResponse = HashMap<u32, String>;

//
// v1/meta/crypto-exchanges
//

#[derive(Clone, Deserialize, Debug)]
pub struct CryptoExchange {
    pub id: u32,
    #[serde(rename = "type")]
    pub exchange_type: Option<String>,
    pub market: String,
    pub name: String,
    pub url: String,
    pub locale: Option<String>,
    pub tier: Option<String>,
}

pub type CryptoCryptoExchangesResponse = Vec<CryptoExchange>;

//
// v2/last/trade/{ticker}
//

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesHistoricTrade {
    pub T: Option<String>,
    pub f: Option<u64>,
    pub q: Option<u64>,
    pub t: Option<u64>,
    pub y: Option<u64>,
    pub c: Option<Vec<u64>>,
    pub e: Option<u64>,
    pub i: Option<String>,
    pub p: Option<f64>,
    pub r: Option<u64>,
    pub s: Option<f64>,
    pub x: Option<u64>,
    pub z: Option<u64>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesHistoricTradesV2Response {
    pub request_id: String,
    pub status: String,
    pub results: StockEquitiesHistoricTrade,
}

pub type StockEquitiesHistoricTradesResponse = StockEquitiesHistoricTradesV2Response;

//
// v2/last/nbbo/{ticker}
//

#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesLastQuoteForASymbolV2Response {
    pub request_id: String,
    pub status: String,
    pub results: StockEquitiesHistoricTrade,
}

pub type StockEquitiesLastQuoteForASymbolResponse = StockEquitiesLastQuoteForASymbolV2Response;

//
// v1/open-close/{ticker}/{date}
//

#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesDailyOpenCloseResponse {
    #[serde(rename = "afterHours")]
    pub after_hours: f64,
    pub close: f64,
    pub from: String,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    #[serde(rename = "preMarket")]
    pub pre_market: f64,
    pub status: String,
    pub symbol: String,
    pub volume: f64,
}

//
// v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to}
//

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesAggregates {
    pub T: Option<String>,
    pub av: Option<u64>,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub n: Option<f64>,
    pub o: f64,
    pub t: Option<u64>,
    pub v: f64,
    pub vw: Option<f64>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesAggregatesResponse {
    pub ticker: String,
    pub adjusted: bool,
    #[serde(rename = "queryCount")]
    pub query_count: u32,
    pub request_id: String,
    #[serde(rename = "resultsCount")]
    pub results_count: u32,
    pub count: u32,
    pub status: String,
    pub results: Vec<StockEquitiesAggregates>,
}

//
// v2/aggs/grouped/locale/{locale}/market/{market}/{date}
//

#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesGroupedDailyResponse {
    pub adjusted: bool,
    #[serde(rename = "queryCount")]
    pub query_count: u32,
    #[serde(rename = "resultsCount")]
    pub results_count: u32,
    pub status: String,
    pub results: Vec<StockEquitiesAggregates>,
}

//
// v2/aggs/ticker/{ticker}/prev
//

#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesPreviousCloseResponse {
    pub ticker: String,
    pub adjusted: bool,
    #[serde(rename = "queryCount")]
    pub query_count: u32,
    #[serde(rename = "resultsCount")]
    pub results_count: u32,
    pub count: u32,
    pub status: String,
    pub results: Vec<StockEquitiesAggregates>,
}

//
// v2/snapshot/locale/{locale}/markets/{market}/tickers
//

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesQuote {
    pub P: f64,
    pub S: u64,
    pub p: f64,
    pub s: u64,
    pub t: u64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesTickerSnapshot {
    pub day: StockEquitiesAggregates,
    #[serde(rename = "lastQuote")]
    pub last_quote: StockEquitiesQuote,
    #[serde(rename = "lastTrade")]
    pub last_trade: StockEquitiesHistoricTrade,
    pub min: StockEquitiesAggregates,
    #[serde(rename = "prevDay")]
    pub prev_day: StockEquitiesAggregates,
    pub ticker: String,
    #[serde(rename = "todaysChange")]
    pub todays_change: f64,
    #[serde(rename = "todaysChangePerc")]
    pub todays_change_perc: f64,
    pub updated: u64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesSnapshotAllTickersResponse {
    pub count: u32,
    pub status: String,
    pub tickers: Vec<StockEquitiesTickerSnapshot>,
}

//
// v2/snapshot/locale/us/markets/stocks/{direction}
//

#[derive(Clone, Deserialize, Debug)]
pub struct StockEquitiesSnapshotGainersLosersResponse {
    pub status: String,
    pub tickers: Vec<StockEquitiesTickerSnapshot>,
}

//
// v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to}
//

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug)]
pub struct ForexEquitiesAggregates {
    pub T: Option<String>,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub n: Option<f64>,
    pub o: f64,
    pub t: Option<u64>,
    pub v: f64,
    pub vw: Option<f64>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ForexCurrenciesAggregatesResponse {
    pub ticker: String,
    #[serde(rename = "queryCount")]
    pub query_count: u32,
    #[serde(rename = "resultsCount")]
    pub results_count: u32,
    pub results: Vec<ForexEquitiesAggregates>,
    pub status: String,
    pub request_id: String,
    pub count: u32,
}

//
// v2/aggs/grouped/locale/global/market/fx/{date}
//

#[derive(Clone, Deserialize, Debug)]
pub struct ForexCurrenciesGroupedDailyResponse {
    #[serde(rename = "queryCount")]
    pub query_count: u32,
    #[serde(rename = "resultsCount")]
    pub results_count: u32,
    pub adjusted: bool,
    pub results: Vec<ForexEquitiesAggregates>,
    pub status: String,
    pub request_id: String,
    pub count: u32,
}

//
// v2/aggs/ticker/{forex_ticker}/prev
//

#[derive(Clone, Deserialize, Debug)]
pub struct ForexCurrenciesPreviousCloseResponse {
    pub ticker: String,
    #[serde(rename = "queryCount")]
    pub query_count: u32,
    #[serde(rename = "resultsCount")]
    pub results_count: u32,
    pub adjusted: bool,
    pub results: Vec<ForexEquitiesAggregates>,
    pub status: String,
    pub request_id: String,
    pub count: u32,
}

//
// v1/open-close/crypto/{from}/{to}/{date}
//

#[derive(Clone, Deserialize, Debug)]
pub struct CryptoOpenTrades {
    pub x: u32,
    pub p: f64,
    pub s: f64,
    pub c: Vec<u32>,
    pub i: String,
    pub t: u64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CryptoDailyOpenCloseResponse {
    pub symbol: String,
    #[serde(rename = "isUTC")]
    pub is_utc: bool,
    pub day: String,
    pub open: f64,
    pub close: f64,
    #[serde(rename = "openTrades")]
    pub open_trades: Vec<CryptoOpenTrades>,
}

//
// v2/aggs/ticker/{cryptoTicker}/range/{multiplier}/{timespan}/{from}/{to}
//

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug)]
pub struct CryptoAggregates {
    pub T: Option<String>,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub n: Option<f64>,
    pub o: f64,
    pub t: Option<u64>,
    pub v: f64,
    pub vw: Option<f64>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CryptoAggregatesResponse {
    pub ticker: String,
    #[serde(rename = "queryCount")]
    pub query_count: u32,
    #[serde(rename = "resultsCount")]
    pub results_count: u32,
    pub results: Vec<CryptoAggregates>,
    pub status: String,
    pub request_id: String,
    pub count: u32,
}

//
// v2/aggs/grouped/locale/global/market/crypto/{date}
//

#[derive(Clone, Deserialize, Debug)]
pub struct CryptoGroupedDailyResponse {
    #[serde(rename = "queryCount")]
    pub query_count: u32,
    #[serde(rename = "resultsCount")]
    pub results_count: u32,
    pub adjusted: bool,
    pub results: Vec<CryptoAggregates>,
    pub status: String,
    pub request_id: String,
    pub count: u32,
}

//
// v2/aggs/ticker/{crypto_ticker}/prev
//

#[derive(Clone, Deserialize, Debug)]
pub struct CryptoPreviousCloseResponse {
    pub ticker: String,
    #[serde(rename = "queryCount")]
    pub query_count: u32,
    #[serde(rename = "resultsCount")]
    pub results_count: u32,
    pub adjusted: bool,
    pub results: Vec<CryptoAggregates>,
    pub status: String,
    pub request_id: String,
    pub count: u32,
}
