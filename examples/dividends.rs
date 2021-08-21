use std::collections::HashMap;
use std::env;

use chrono::prelude::*;
use chrono::Duration;

use polygon_client::client::Client;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        println!("Usage: dividends <ticker1> <ticker2> <ticker3> ...");
        return;
    }

    let one_year_ago = (Local::now() - Duration::days(365)).date();

    let client = Client::new(None, None);

    for ticker in args.iter() {
        let query_params = HashMap::new();
        let dividends = client
            .reference_stock_dividends(ticker, &query_params)
            .await;
        if dividends.is_ok() {
            let dividends_ref = &dividends.unwrap();
            let res = dividends_ref
                .results
                .iter()
                .filter_map(|x| {
                    let ex_date = NaiveDate::parse_from_str(&x.ex_date, "%Y-%m-%d").unwrap();

                    if ex_date > one_year_ago.naive_local() {
                        Some(x)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if res.len() > 0 {
                let previous_close_res = client
                    .stock_equities_previous_close(ticker, &query_params)
                    .await
                    .expect(&format!(
                        "unable to find previous close for ticker {}",
                        ticker
                    ));

                if previous_close_res.results.len() == 0 {
                    panic!("no previous close found for ticker {}", ticker);
                }

                let close = previous_close_res.results.first().unwrap().c;
                let sum: f64 = res.iter().map(|d| d.amount).sum();

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
