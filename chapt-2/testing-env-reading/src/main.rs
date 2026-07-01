use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let address = match dotenvy::var("WALLET_ADDRESS") {
        Ok(address) => address,
        Err(e) => panic!("Error: {}", e),
    };

    println!("The Wallet address is: {}", address);
}
