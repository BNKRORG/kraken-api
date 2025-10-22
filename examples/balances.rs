use kraken_api::auth::KrakenAuth;
use kraken_api::client::KrakenClient;

#[tokio::main]
async fn main() {
    let auth = KrakenAuth::api_keys("<api-key>", "<secret>");

    let client = KrakenClient::new(auth).unwrap();

    // let balances = client.balances().await.unwrap();

    // if balances.is_empty() {
    //     println!("No balances");
    //     return;
    // }
    //
    // for (coin, amount) in balances {
    //     println!("{coin}: {amount}");
    // }

    let balance = client.btc_balance().await.unwrap();

    println!("Balance: {balance} BTC");
}
