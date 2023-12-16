mod eth_wallet;
use eth_wallet::EthWallet;

fn main() {
    let wallet = EthWallet::new();

    // If you want to see the private key, uncomment the line below
    // println!("Private Key: {}", wallet.secret_key_hex());
    println!("Private Key: {:?}", wallet.secret_key);
    println!("Public Key: {}", wallet.public_key);
    println!(
        "Address: 0x{}",
        wallet
            .address
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );
}
