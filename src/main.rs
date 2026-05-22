#![allow(unused_imports)]
#![allow(dead_code)]

mod helpers;
mod measuring_functions;

use std::error::Error;
use subxt::config::Header;
use subxt::{OnlineClient, PolkadotConfig};
use subxt::backend::rpc::RpcClient;

use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt_signer::sr25519::Keypair as Sr25519Keypair;
use subxt_signer::ecdsa::Keypair as EcdsaKeypair;
use subxt_signer::SecretUri;

use crate::helpers::*;
use crate::measuring_functions::*;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "ws://127.0.0.1:9944";

// 1. Connect to node, this snippet will check if a node is running and can be connected 

    let rpc_client = RpcClient::from_url(url).await?;
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc_client.clone()).await?;

    println!("✅ Connected to node");

// 2. Get latest block info: This will show that a Blockchain can be connected to get basic information like Block number and hash of the block
    
    let block = api.blocks().at_latest().await?;
    let header = block.header();

    println!("📦 Block number: {}", header.number);
    println!("🔗 Block hash: 0x{}", hex::encode(block.hash().as_ref()));
    
// 3. Generate an account using a seed using Sr25519 and Ecdsa Signature algorithms

    let seed32: [u8; 32] = *b"12345678901234567890123456789012";
    let sr = Sr25519Keypair::from_secret_key(seed32)?;
    let sr_sender = sr25519_account_id(&sr);

    let ec = EcdsaKeypair::from_secret_key(seed32)?;
    let ec_sender = ecdsa_account_id(&ec);

    print_crypto_sizes(&sr, &ec, &sr_sender, &ec_sender);

    println!("Sr25519:   {}", sr_sender);
    println!("ECDSA:     {}", ec_sender);
        

// 4. Funding the accounts with some coins: For reference 1 coin = 1_000_000_000_000 units

    let alice = Sr25519Keypair::from_uri(&SecretUri::from_str("//Alice")?)?;
    let alice_account: AccountId32 = alice.public_key().into();

    let amount: u128 = 10_000_000;

    println!("funding sr...");
    fund_account(&api, &alice, &sr_sender).await?;
    println!("funding sr OK");

    println!("funding ecdsa...");
    fund_account(&api, &alice, &ec_sender).await?;
    println!("funding ecdsa OK");
    
 // 5. Performance evaluation measurement; Ledger Latency, here the code will run for 10 iterations
 
    measure_ledger_latency(&api, &sr, &sr_sender, &alice_account, amount, 10, "sr25519_measure_transaction_latency.csv",).await?;

    measure_ledger_latency(&api, &ec, &ec_sender, &alice_account, amount, 10, "ecdsa_measure_transaction_latency.csv",).await?;
    
  // 6. Performance evaluation measurement; Key Generation from Seed to Kep Pair Generation
 
    measure_keygen_seed_to_keypair("Sr25519", 10_000, "sr25519_keygen.csv", |i| {
        let mut seed = seed32;
        seed[0] ^= (i as u8);
        let kp = Sr25519Keypair::from_secret_key(seed)?;
        std::hint::black_box(kp);
        Ok(())
    })?;

    measure_keygen_seed_to_keypair("ECDSA", 10_000, "ecdsa_keygen.csv", |i| {
        let mut seed = seed32;
        seed[0] ^= (i as u8);
        let kp = EcdsaKeypair::from_secret_key(seed)?;
        std::hint::black_box(kp);
        Ok(())
    })?; 
    
  // 7. Performance evaluation measurement: Transaction Signing Latency
   
    measure_signing_time(&api, &sr, &sr_sender, &alice_account, amount, 10_000, "sr25519_sign_latency.csv", "sr25519").await?;

    measure_signing_time(&api, &ec, &ec_sender, &alice_account, amount, 10_000, "ecdsa_sign_latency.csv", "ecdsa").await?;


  // 8. Performance evaluation measurement:   Transaction Size and Weight 
    measure_extrinsic_size_and_weight(
        &api, &rpc_client, &sr, &sr_sender, &alice_account, amount,
        "sr25519", 1000, "sr25519_xt_size_weight.csv"
    ).await?;

    measure_extrinsic_size_and_weight(
        &api, &rpc_client, &ec, &ec_sender, &alice_account, amount,
        "ecdsa", 1000, "ecdsa_xt_size_weight.csv"
    ).await?;

   
    Ok(())
}
