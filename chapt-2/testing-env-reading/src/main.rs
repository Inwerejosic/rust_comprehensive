use dotenvy::dotenv;

const ANSI_GREEN: &str = "\x1b[32m";

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let address = dotenvy::var("WALLET_ADDRESS")?;

    println!("The Wallet address is: {}{}", ANSI_GREEN, address);

    Ok(())
}
