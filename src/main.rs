use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug)]
struct Ticker {
    ticker: String,
    shares: f32,
    apply_drip: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tickers {
    tickers: Vec<Ticker>,
    dividend_projection_in_years: u8, // Makes no sense to have a bigger type unless you plan on living more than 256 years change if necessary 😆
    yearly_growth_percentage : u8, // 0-100 %
    div_yearly_growth_percentage : u8 // 0 - 100 %
}

/*
{
 "ticker": "VOO",
 "queryCount": 1,
 "resultsCount": 1,
 "adjusted": true,
 "results": [
  {
   "T": "VOO",
   "v": 10458586,
   "vw": 409.0775,
   "o": 413.15,
   "c": 404.94,
   "h": 415.2,
   "l": 403.57,
   "t": 1644613200000,
   "n": 204637
  }
 ],
 "status": "OK",
 "request_id": "fe1afce23b22c8c5f4d71afa91aa8e07",
 "count": 1
}*/
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Result {
    T: String,
    v: f64,
    vw: f64,
    o: f64,
    c: f64,
    h: f64,
    l: f64,
    t: u64,
    n: u64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct PreviousClose {
    ticker: String,
    queryCount: u8,
    resultsCount: u8,
    adjusted: bool,
    results: Vec<Result>,
    status: String,
    request_id: String,
    count: u8,
}

/*
{
 {
    "results": [{
        "cash_amount": 1.5329,
        "declaration_date": "2021-12-17",
        "dividend_type": "CD",
        "ex_dividend_date": "2021-12-21",
        "frequency": 4,
        "pay_date": "2021-12-27",
        "record_date": "2021-12-22",
        "ticker": "VOO"
    }],
    "status": "OK",
    "request_id": "f6b17048a72baf1279e4fcea602cb6e8",
    "next_url": "https://api.polygon.io/v3/reference/dividends?cursor=YXA9MTEzNzMmYXM9JmxpbWl0PTEmb3JkZXI9ZGVzYyZzb3J0PWV4X2RpdmlkZW5kX2RhdGUmdGlja2VyPVZPTw"
}
*/

#[derive(Serialize, Deserialize, Debug)]
struct ResultDiv {
    cash_amount: f64,
    declaration_date: String,
    dividend_type: String,
    ex_dividend_date: String,
    frequency: u8,
    pay_date: String,
    record_date: String,
    ticker: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DividendsV3 {
    results: Vec<ResultDiv>,
    status: String,
    request_id: String,
    next_url: String,
}

fn main() {
    println!("💵 💵 💵 Analyzing porfolio.json 💵 💵 💵");
    if let Ok(data) = read_to_string("porfolio.json") {
        if let Ok(tickers) = serde_json::from_str::<Tickers>(data.as_str()) {
            let mut total = 0.0;
            for ticker in tickers.tickers {
                total += analyse_stock(&ticker, tickers.dividend_projection_in_years, ticker.apply_drip, 1.0 + ((tickers.yearly_growth_percentage as f64) / 100.0), 1.0 + ((tickers.div_yearly_growth_percentage as f64) / 100.0));
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            println!("In {} years your stock profolio may be worth ${:.2}", tickers.dividend_projection_in_years, total);
        } else {
            println!("Invalid porfolio file!");
        }
    } else {
        println!("Missing porfolio.json");
    }
    println!("💵 💵 💵 Finished analysing 💵 💵 💵")
}

fn analyse_stock(ticker: &Ticker, years : u8, apply_drip : bool, stock_yield : f64, div_yield : f64) -> f64 {
    print!("[ DRIP = {} Stock Yield =  {} Div Yield = {} ] ", apply_drip, stock_yield, div_yield);
    let dividend_rest_url = format!("https://api.polygon.io/v3/reference/dividends?ticker={}&limit=1&apiKey=EvSs_0FZQ91dpD92_xmPWvdWH2VGJReg", ticker.ticker);
    let price_rest_url = format!("https://api.polygon.io/v2/aggs/ticker/{}/prev?adjusted=true&apiKey=EvSs_0FZQ91dpD92_xmPWvdWH2VGJReg", ticker.ticker);
    if let Ok(response1) = reqwest::blocking::get(dividend_rest_url) {
        if let Ok(body) = response1.text() {
            if let Ok(response2) = reqwest::blocking::get(price_rest_url) {
                if let Ok(body2) = response2.text() {
                    if let Ok(price) = serde_json::from_str::<PreviousClose>(body2.as_str()) {
                        let result = &price.results[0];
                        let mut shares = ticker.shares as f64;
                        let mut price = result.c;
                        let mut principal = price * (ticker.shares as f64);
                        let original_principal = principal;
                        if let Ok(dividend) = serde_json::from_str::<DividendsV3>(body.as_str()) {
                            let div_info = &dividend.results[0];
                            let mut div_per_share = div_info.cash_amount;
                            for _ in 0..years {
                                for _ in 0..div_info.frequency {
                                    let new_amount = shares * div_per_share;
                                    let new_amount_shares = new_amount / price;
                                    if apply_drip {
                                        shares += new_amount_shares;
                                        principal = shares * price;
                                    }
                                    else {
                                        principal = (shares * price) + new_amount;
                                    }
                                }
                                price *= stock_yield;
                                div_per_share *= div_yield;
                            }
                            println!(
                                "Your {} will start ${:.2} and in 35 years will be ${:.2}!",
                                ticker.ticker, original_principal, principal
                            );
                        } else {
                            println!(
                                "Failed to parse json from stock API on ticker {}!",
                                ticker.ticker
                            );
                            return 0.0;
                        }
                        principal
                    } else {
                        println!(
                            "Failed to parse json from stock API on ticker {}!",
                            ticker.ticker
                        );
                        0.0
                    }
                } else {
                    println!("Failed to get price info from {}", ticker.ticker);
                    0.0
                }
            } else {
                println!("Failed to get price info from {}", ticker.ticker);
                0.0
            }
        } else {
            println!("Failed to get dividend info from {}", ticker.ticker);
            0.0
        }
    } else {
        println!("Failed to get dividend info from {}", ticker.ticker);
        0.0
    }
}
