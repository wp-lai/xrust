use clap::{arg, App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("coinprice")
        .about("Fetch coin price from CoinMarketCap")
        .arg(arg!(-c --coin <coin> "coin symbol"))
        .get_matches();
    let coin = matches.value_of("coin").unwrap().to_uppercase();

    let client = reqwest::Client::new();
    let resp: serde_json::Value = client
        .get(" https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY", std::env::var("CMC_API")?)
        .query(&[("symbol", &coin)])
        .send()
        .await?
        .json()
        .await?;

    let price = &resp["data"][&coin]["quote"]["USD"]["price"]
        .as_f64()
        .unwrap();

    println!("current price of {} is ${:.2}", coin, price);

    Ok(())
}
