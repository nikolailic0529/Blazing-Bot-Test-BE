use anyhow::Result;
use num_bigint::BigUint;
use tonlib::client::TonClientInterface;
use std::time::SystemTime;
use std::sync::Arc;

use tonlib::address::TonAddress;
use tonlib::cell::Cell;
use tonlib::cell::BagOfCells;
use tonlib::message::TransferMessage;

use tonlib::config::TESTNET_CONFIG;
use tonlib::client::TonClient;
use tonlib::client::TonClientBuilder;
use tonlib::client::TonConnectionParams;
use tonlib::contract::TonContractFactory;

use tonlib::mnemonic::Mnemonic;
use tonlib::mnemonic::KeyPair;

use tonlib::wallet::{TonWallet, WalletVersion};
use tonlib::contract::{JettonMasterContract, TonWalletContract};

async fn create_client()-> Result<TonClient>{
    TonClient::set_log_verbosity_level(2); //setup of logging level

    let client = TonClientBuilder::new()
        .with_pool_size(10)
        .with_keystore_dir(String::from("/tmp"))
        .build()
        .await?;
    Ok(client)
}

async fn create_testnet_client()-> Result<TonClient>{
    TonClient::set_log_verbosity_level(2); //setup of logging level

    let client = TonClientBuilder::new()
        .with_connection_params(&TonConnectionParams {
            config: TESTNET_CONFIG.to_string(),
            blockchain_name: None,
            use_callbacks_for_network: false,
            ignore_cache: false,
            keystore_dir: None,
            notification_queue_length: 10000,
            concurrency_limit: 100,
        })
        .with_pool_size(10)
        .with_keystore_dir(String::from("/tmp"))
        .build()
        .await?;

    Ok(client)
}

async fn create_key_pair() -> Result<KeyPair> {
    let mnemonic = Mnemonic::new(
        vec![
            "mirror", "spice", "begin", "hurry", "shrug", "upon", "kite", "tray", "awake", "embark", "dutch", "wall",
            "plate", "tape", "mimic", "pigeon", "virus", "raw", "faith", "student", "like", "crane", "indoor", "canvas",
        ],
        &None,
    )?;
    let key_pair = mnemonic.to_key_pair()?;

    Ok(key_pair)
}

async fn create_simple_transfer() -> Result<()> {
    // let client = create_client().await?;
    let client = create_testnet_client().await?;
    let contract_factory = TonContractFactory::builder(&client).build().await?;
    let key_pair = create_key_pair().await?;
    let wallet = TonWallet::derive_default(WalletVersion::V3R2, &key_pair)?;
    let wallet_contract = contract_factory.get_contract(&wallet.address);
    let seqno =  wallet_contract.seqno().await?;

    println!("Sender: {:?}", wallet.address);

    // let dest: TonAddress = "UQBLJg9IzDgBVzXN5-y_8yljtjkhk-9Wc3VDKcDBJWq_3JJs".parse()?;
    let dest: TonAddress = "0QAfX8cOX7tBdjHysYkhQqov1a_W8aiF765EICGQkcl1fKpR".parse()?;
    
    println!("Dest: {:?}", dest.to_base64_url());

    let value = BigUint::from(10000000u64); // 0.01 TON
    let transfer = TransferMessage::new(&dest, &value).build()?;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs() as u32;
    let transfer_arc: Vec<Arc<Cell>> = vec![transfer].into_iter().map(Arc::new).collect();
    let msg_cell = wallet.create_external_message(now + 60, seqno, transfer_arc, false)?;
    let boc = BagOfCells::from_root(msg_cell);
    let tx = boc.serialize(true)?;
    let hash = client.send_raw_message_return_hash(tx.as_slice()).await?;

    println!("{:?}", hash);

    Ok(())
}

async fn get_state()-> Result<()>{
    let client = create_client().await?;
    // let client = create_testnet_client().await?;

    println!("Client is created");

    // TonClient::set_log_verbosity_level(2); //setup of logging level
    // let client = TonClient::builder().build().await?;
    let address = TonAddress::from_base64_url(
        "UQB3G_lRw7SW0MFMijC5YWZTIEW97kvEXmeXZAaI9m02k5Ts",
        // "0QDwYdY0qWYFRXTurfycf-BHhQwQ30q817nhjIDYoOHgzRii",
    )?;

    println!("Address: {:?}", address.to_base64_url());

    let r = client
            .get_account_state(&address)
            .await?;

    println!("{:?}", r);
    Ok(())
}

#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    
    // let _ = get_state().await;
    let _ = create_simple_transfer().await;
}
