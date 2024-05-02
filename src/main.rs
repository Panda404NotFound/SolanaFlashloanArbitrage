use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use mango_v4::state::MangoGroup;
use mango_v4::instruction::flash_loan::{FlashLoanBegin, FlashLoanEnd};

fn main() {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);

    let mango_group_pk = Pubkey::new("Ec2enZyoC4nGpEfu2sUNAa2nUGJHWxoUWYSEJ2hNTWTA");
    let mango_group = MangoGroup::load_checked(&client, mango_group_pk).unwrap();
    
    let payer = solana_sdk::signer::keypair::read_keypair_file("~/.config/solana/devnet.json").unwrap();
    let wallet_pk = payer.pubkey();

    println!("Wallet balance before: {}", client.get_balance(&wallet_pk).unwrap());
    
    let flash_loan_ix = FlashLoanBegin {
        amount: 1_000_000,
        token_account: mango_group.tokens[0].vault,
    };

    let end_loan_ix = FlashLoanEnd {};

    let instructions = vec![
        flash_loan_ix,
        end_loan_ix  
    ];

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&wallet_pk),
        &[&payer],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&tx).unwrap();
    println!("Flash loan tx: https://explorer.solana.com/tx/{}?cluster=devnet", signature);

    println!("Wallet balance after: {}", client.get_balance(&wallet_pk).unwrap());
}