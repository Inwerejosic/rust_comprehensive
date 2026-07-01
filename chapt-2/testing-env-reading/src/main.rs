use dotenvy::dotenv;

const ANSI_GREEN: &str = "\x1b[32m";
// const ANSI_RESET: &str = "\x1b[0m";

fn main() {
    dotenv().ok();

    let address = match dotenvy::var("WALLET_ADDRESS") {
        Ok(address) => address,
        Err(e) => panic!("Error: {}", e),
    };

    println!("The Wallet address is: {}{}", ANSI_GREEN, address);
}
