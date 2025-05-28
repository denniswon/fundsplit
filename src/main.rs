use alloy::network::TransactionBuilder;
use alloy::primitives::utils::{format_ether, parse_ether};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::primitives::{Address, U256};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use std::{env, fs::File, io::{BufRead, BufReader}, time::Duration};
use tokio::time::sleep;

#[derive(Parser)]
#[command(name = "fundsplit")]
#[command(version = "0.1.0")]
#[command(about = r#"
|-----------------------------------------------------------------------------|
|      _______  _______  __   __     _______  _______  _______  ______        |
|     |       ||       ||  | |  |   |       ||       ||       ||    _ |       |
|     |    ___||   _   ||  |_|  |   |    ___||    ___||    ___||   | ||       |
|     |   |___ |  | |  ||       |   |   |___ |   |___ |   |___ |   |_||_      |
|     |    ___||  |_|  ||       |   |    ___||    ___||    ___||    __  |     |
|     |   |    |       | |     |    |   |    |   |    |   |___ |   |  | |     |
|     |___|    |_______|  |___|     |___|    |___|    |_______||___|  |_|     |
|                                                                             |
|           FundSplit CLI — Send ETH to a list of addresses                    |
|-----------------------------------------------------------------------------|
"#)]
#[command(after_help = r#"
                       ╔═══════════════════════════════╗
                       ║        COMPLETE               ║
                       ╚═══════════════════════════════╝
"#)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Airdrop ETH to a list of addresses
    Drop(DropArgs),

    /// Fetches the balances of a list of addresses
    Balances(BalancesArgs),
}

/// ETH Airdrop CLI on Base Mainnet
#[derive(Parser, Debug)]
#[command(name = "eth-airdrop", version, about = "Send ETH to a list of addresses")]
struct DropArgs {
    /// ETH amount to send per recipient (e.g. 0.25)
    #[arg(
        short = 'a',
        long = "amount",
        default_value = "0.25",
        value_parser = parse_ether,
    )]
    amount: U256,

    /// Path to the recipients.txt file
    #[arg(
        short = 'i',
        long = "input-file",
        default_value = "recipients.txt",
    )]
    input_file: String,
}

#[derive(Parser, Debug)]
#[command(name = "eth-balances", version, about = "Fetch the balances of a list of addresses")]
struct BalancesArgs {
    /// Path to the recipients.txt file
    #[arg(
        short = 'i',
        long = "input-file",
        default_value = "recipients.txt",
    )]
    input_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let rpc_url = env::var("RPC_URL").context("Missing RPC_URL")?;
    let pk = env::var("MAIN_PRIVATE_KEY").context("Missing MAIN_PRIVATE_KEY")?;
    let wallet = pk.parse::<PrivateKeySigner>().context("Invalid private key")?;
    let sender = wallet.address();

    let rpc_url = rpc_url.parse().context("Invalid RPC URL")?;
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_http(rpc_url);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Drop(args) => {
            let file = File::open(&args.input_file).context("Failed to open recipients file")?;
            let reader = BufReader::new(file);
            let recipients = reader
                .lines()
                .filter_map(|line| line.ok())
                .filter_map(|addr| addr.as_str().parse::<Address>().ok())
                .collect::<Vec<_>>();

            println!("🚀 Sending {} ETH to {} recipients...\n", format_ether(args.amount), recipients.len());

            for recipient in recipients.clone() {
                println!("→ Sending to {:?}", recipient);

                let tx_request = TransactionRequest::default()
                    .with_nonce(provider.get_transaction_count(sender).await?)
                    .with_from(sender)
                    .with_to(recipient)
                    .with_value(args.amount);

                match provider.send_transaction(tx_request).await {
                    Ok(pending) => {
                        let receipt = pending
                            .with_required_confirmations(2)
                            .with_timeout(Some(Duration::from_secs(180)))
                            .get_receipt()
                            .await;

                        match receipt {
                            Ok(receipt) => {
                                println!("   ✅ Tx Hash: {:?}", receipt.transaction_hash);
                            }
                            Err(e) => {
                                eprintln!("   ❌ Pending tx error: {}", e);
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("   ❌ Transaction error: {}", e);
                        continue;
                    }
                }
                sleep(Duration::from_millis(300)).await;
            }

            println!("\n📦 Final Balances:");
            for addr in recipients {
                match provider.get_balance(addr).await {
                    Ok(bal) => {
                        let bal_eth = format_ether(bal);
                        println!("{:?} → {} ETH", addr, bal_eth);
                    }
                    Err(e) => {
                        eprintln!("   ❌ Error getting balance for address {:?}: {}", addr, e);
                        continue;
                    }
                }
            }
        }
        Commands::Balances(args) => {
            let file = File::open(&args.input_file).context("Failed to open recipients file")?;
            let reader = BufReader::new(file);
            let recipients = reader
                .lines()
                .filter_map(|line| line.ok())
                .filter_map(|addr| addr.as_str().parse::<Address>().ok())
                .collect::<Vec<_>>();

            println!("\n📦 Final Balances:");
            for addr in recipients {
                match provider.get_balance(addr).await {
                    Ok(bal) => {
                        let bal_eth = format_ether(bal);
                        println!("{:?} → {} ETH", addr, bal_eth);
                    }
                    Err(e) => {
                        eprintln!("   ❌ Error getting balance for address {:?}: {}", addr, e);
                        continue;
                    }
                }
            }
        }
    }

    Ok(())
}
