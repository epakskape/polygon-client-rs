use std::collections::HashMap;
use std::env;

use chrono::prelude::*;
use chrono::Duration;

use polygon_client::rest::RESTClient;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        println!("Usage: dividends <ticker1> <ticker2> <ticker3> ...");
        return;
    }

    let one_year_ago = (Local::now() - Duration::days(365)).date();

    let client = RESTClient::new(None, None);

    for ticker in args.iter() {
        let query_params = HashMap::new();
        let dividends = client
            .reference_stock_dividends(ticker, &query_params)
            .await;
        if let Ok(dividends_ref) = dividends {
            let res = dividends_ref
                .results
                .iter()
                .filter(|&x| {
                    NaiveDate::parse_from_str(&x.ex_dividend_date, "%Y-%m-%d").unwrap()
                        > one_year_ago.naive_local()
                })
                .collect::<Vec<_>>();

            if !res.is_empty() {
                let previous_close_res = client
                    .stock_equities_previous_close(ticker, &query_params)
                    .await
                    .unwrap_or_else(|_| {
                        panic!("unable to find previous close for ticker {}", ticker)
                    });

                if previous_close_res.results.is_empty() {
                    panic!("no previous close found for ticker {}", ticker);
                }

                let close = previous_close_res.results.first().unwrap().c;
                let sum: f64 = res.iter().map(|d| d.cash_amount).sum();

                println!("Yield for {} is {:.2}% [previous close = {}, sum of last {} dividends = {:.2}]",
                    ticker,
                    (sum / close) * 100f64,
                    close,
                    res.len(),
                    sum);
            }
        }
    }
}
