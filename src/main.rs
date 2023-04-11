mod args;
mod transaction_data;
mod usage;

use args::Command;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;
use zerocopy::AsBytes;

const VAMP_PROGRAM_PUBKEY : &str = "vamp3angna1CBRcV6KqoxyaYw3mPybHEeoPLtmpS99N";
const SYSTEM_PROGRAM_PUBKEY : &str = "11111111111111111111111111111111";
const VOTE_PROGRAM_PUBKEY : &str = "Vote111111111111111111111111111111111111111";
const CLOCK_SYSVAR_PUBKEY : &str = "SysvarC1ock11111111111111111111111111111111";

const LAMPORTS_PER_SOL : f64 = 1e9;

fn error_exit(err : &str) -> !
{
    eprintln!("{}\n", err);

    eprintln!("Try 'solana-vamp help' for help.");

    std::process::exit(-1)
}

#[cfg(target_endian = "big")]
fn exit_if_big_endian()
{
    error_exit("solana-vamp is currently unsupported on big endian CPU architectures, sorry.");
}

#[cfg(not(target_endian = "big"))]
fn exit_if_big_endian()
{
}

fn main()
{
    // At the moment this program doesn't work on big endian because of the byte ordering of as_bytes data.
    // It's unclear if there will ever be a big endian system that solana-vamp is compiled for, so punting on
    // the issue for now.
    exit_if_big_endian();

    let (fee_payer, rpc_url, commitment, vote_account, command) = args::parse_command();

    let commitment = commitment
        .as_ref()
        .map(|commitment| CommitmentConfig::from_str(commitment).expect(&format!("Invalid commitment {}", commitment)))
        .unwrap_or(CommitmentConfig::default());

    let rpc_client = RpcClient::new_with_commitment(rpc_url, commitment);

    // Load fee_payer keypair
    let fee_payer = fee_payer.map(|fee_payer| load_keypair(&fee_payer, "fee payer"));

    // vamp program pubkey
    let program_id = Pubkey::from_str(VAMP_PROGRAM_PUBKEY).unwrap();

    // vote_account pubkey
    let vote_account = make_pubkey(&vote_account, "vote account");

    // Compute the vote account manager account as a PDA of the program, with the seed being the vote account
    let (vote_account_manager_state_account, _) =
        Pubkey::find_program_address(&[vote_account.to_bytes().as_slice()], &program_id);

    match command {
        Command::Enter { withdraw_authority, administrator, max_commission, max_commission_increase_per_epoch } => {
            let fee_payer = fee_payer.unwrap();

            let withdraw_authority = load_keypair(&withdraw_authority, "withdraw authority");

            let accounts = vec![
                // Vote Account Manager State Account
                AccountMeta { pubkey : vote_account_manager_state_account, is_signer : false, is_writable : true },
                // Vote Account
                AccountMeta { pubkey : vote_account, is_signer : false, is_writable : true },
                // Funding Account
                AccountMeta { pubkey : fee_payer.pubkey(), is_signer : true, is_writable : true },
                // Current Withdraw Authority
                AccountMeta { pubkey : withdraw_authority.pubkey(), is_signer : true, is_writable : false },
                // System Program Id
                AccountMeta {
                    pubkey : Pubkey::from_str(SYSTEM_PROGRAM_PUBKEY).unwrap(),
                    is_signer : false,
                    is_writable : false
                },
                // Vote Program Id
                AccountMeta {
                    pubkey : Pubkey::from_str(VOTE_PROGRAM_PUBKEY).unwrap(),
                    is_signer : false,
                    is_writable : false
                },
                // Clock Sysvar Id
                AccountMeta {
                    pubkey : Pubkey::from_str(CLOCK_SYSVAR_PUBKEY).unwrap(),
                    is_signer : false,
                    is_writable : false
                },
            ];

            let data = transaction_data::EnterData {
                instruction_code : 0,

                administrator : make_pubkey(&administrator, "administrator").to_bytes(),

                use_commission_caps : max_commission.is_some() || max_commission_increase_per_epoch.is_some(),

                max_commission : max_commission.unwrap_or(0),

                max_commission_increase_per_epoch : max_commission_increase_per_epoch.unwrap_or(0)
            };

            let instruction = Instruction { program_id, accounts, data : AsBytes::as_bytes(&data).to_vec() };

            let tx = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&fee_payer.pubkey()),
                &[&fee_payer, &withdraw_authority],
                get_latest_blockhash(&rpc_client)
            );

            submit_transaction(&rpc_client, &tx)
        },
        Command::SetLeaveEpoch { withdraw_authority, leave_epoch } => {
            let fee_payer = fee_payer.unwrap();

            let withdraw_authority = load_keypair(&withdraw_authority, "withdraw authority");

            let accounts = vec![
                // Vote Account Manager State Account
                AccountMeta { pubkey : vote_account_manager_state_account, is_signer : false, is_writable : true },
                // Vote Account
                AccountMeta { pubkey : vote_account, is_signer : false, is_writable : true },
                // Withdraw Authority
                AccountMeta { pubkey : withdraw_authority.pubkey(), is_signer : true, is_writable : false },
            ];

            let data = transaction_data::SetLeaveEpochData { instruction_code : 1, padding : [0_u8; 7], leave_epoch };

            let instruction = Instruction { program_id, accounts, data : AsBytes::as_bytes(&data).to_vec() };

            let tx = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&fee_payer.pubkey()),
                &[&fee_payer, &withdraw_authority],
                get_latest_blockhash(&rpc_client)
            );

            submit_transaction(&rpc_client, &tx)
        },
        Command::Leave { withdraw_authority } => {
            let fee_payer = fee_payer.unwrap();

            let withdraw_authority = load_keypair(&withdraw_authority, "withdraw authority");

            let accounts = vec![
                // Vote Account Manager State Account
                AccountMeta { pubkey : vote_account_manager_state_account, is_signer : false, is_writable : true },
                // Vote Account
                AccountMeta { pubkey : vote_account, is_signer : false, is_writable : true },
                // Withdraw Authority
                AccountMeta { pubkey : withdraw_authority.pubkey(), is_signer : true, is_writable : false },
                // Lamports Recipient -- assume fee payer
                AccountMeta { pubkey : fee_payer.pubkey(), is_signer : false, is_writable : true },
                // Vote Program Id
                AccountMeta {
                    pubkey : Pubkey::from_str(VOTE_PROGRAM_PUBKEY).unwrap(),
                    is_signer : false,
                    is_writable : false
                },
                // Clock Sysvar Id
                AccountMeta {
                    pubkey : Pubkey::from_str(CLOCK_SYSVAR_PUBKEY).unwrap(),
                    is_signer : false,
                    is_writable : false
                },
            ];

            let data = transaction_data::LeaveData { instruction_code : 2 };

            let instruction = Instruction { program_id, accounts, data : AsBytes::as_bytes(&data).to_vec() };

            let tx = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&fee_payer.pubkey()),
                &[&fee_payer, &withdraw_authority],
                get_latest_blockhash(&rpc_client)
            );

            submit_transaction(&rpc_client, &tx)
        },
        Command::SetAdministrator { withdraw_authority, administrator } => {
            set_authority(
                rpc_client,
                program_id,
                vote_account,
                vote_account_manager_state_account,
                fee_payer.unwrap(),
                3,
                &withdraw_authority,
                "withdraw authority",
                &administrator,
                "administratorn"
            );
        },
        Command::SetOperationalAuthority { administrator, authority } => {
            set_authority(
                rpc_client,
                program_id,
                vote_account,
                vote_account_manager_state_account,
                fee_payer.unwrap(),
                4,
                &administrator,
                "administrator",
                &authority,
                "operational authority"
            );
        },
        Command::SetRewardsAuthority { administrator, authority } => {
            set_authority(
                rpc_client,
                program_id,
                vote_account,
                vote_account_manager_state_account,
                fee_payer.unwrap(),
                5,
                &administrator,
                "administrator",
                &authority,
                "rewards authority"
            );
        },
        Command::SetVoteAuthority { operational_authority, authority } => {
            let fee_payer = fee_payer.unwrap();

            let operational_authority = load_keypair(&operational_authority, "operational authority");

            let new_authority = make_pubkey(&authority, "vote authority");

            let accounts = vec![
                // Vote Account Manager State Account
                AccountMeta { pubkey : vote_account_manager_state_account, is_signer : false, is_writable : false },
                // Vote Account
                AccountMeta { pubkey : vote_account, is_signer : false, is_writable : true },
                // Operational Authority
                AccountMeta { pubkey : operational_authority.pubkey(), is_signer : true, is_writable : false },
                // Vote Program Id
                AccountMeta {
                    pubkey : Pubkey::from_str(VOTE_PROGRAM_PUBKEY).unwrap(),
                    is_signer : false,
                    is_writable : false
                },
                // Clock Sysvar Id
                AccountMeta {
                    pubkey : Pubkey::from_str(CLOCK_SYSVAR_PUBKEY).unwrap(),
                    is_signer : false,
                    is_writable : false
                },
            ];

            let data = transaction_data::SetVoteAuthorityData {
                instruction_code : 6,
                new_authority : new_authority.to_bytes()
            };

            let instruction = Instruction { program_id, accounts, data : AsBytes::as_bytes(&data).to_vec() };

            let tx = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&fee_payer.pubkey()),
                &[&fee_payer, &operational_authority],
                get_latest_blockhash(&rpc_client)
            );

            submit_transaction(&rpc_client, &tx)
        },
        Command::SetValidatorIdentity { operational_authority, validator_identity } => {
            let fee_payer = fee_payer.unwrap();

            let operational_authority = load_keypair(&operational_authority, "operational authority");

            let new_identity = load_keypair(&validator_identity, "validator identity");

            let accounts = vec![
                // Vote Account Manager State Account
                AccountMeta { pubkey : vote_account_manager_state_account, is_signer : false, is_writable : false },
                // Vote Account
                AccountMeta { pubkey : vote_account, is_signer : false, is_writable : true },
                // Operational Authority
                AccountMeta { pubkey : operational_authority.pubkey(), is_signer : true, is_writable : false },
                // New Validator Identity
                AccountMeta { pubkey : new_identity.pubkey(), is_signer : true, is_writable : false },
                // Vote Program Id
                AccountMeta {
                    pubkey : Pubkey::from_str(VOTE_PROGRAM_PUBKEY).unwrap(),
                    is_signer : false,
                    is_writable : false
                },
            ];

            let data = transaction_data::SetValidatorIdentityData { instruction_code : 7 };

            let instruction = Instruction { program_id, accounts, data : AsBytes::as_bytes(&data).to_vec() };

            let tx = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&fee_payer.pubkey()),
                &[&fee_payer, &operational_authority, &new_identity],
                get_latest_blockhash(&rpc_client)
            );

            submit_transaction(&rpc_client, &tx)
        },
        Command::Withdraw { rewards_authority, recipient, amount } => {
            let fee_payer = fee_payer.unwrap();

            let rewards_authority = load_keypair(&rewards_authority, "rewards authority");

            let recipient = make_pubkey(&recipient, "recipient");

            let accounts = vec![
                // Vote Account Manager State Account
                AccountMeta { pubkey : vote_account_manager_state_account, is_signer : false, is_writable : false },
                // Vote Account
                AccountMeta { pubkey : vote_account, is_signer : false, is_writable : true },
                // Rewards Authority
                AccountMeta { pubkey : rewards_authority.pubkey(), is_signer : true, is_writable : false },
                // Recipient
                AccountMeta { pubkey : recipient, is_signer : false, is_writable : true },
                // Vote Program Id
                AccountMeta {
                    pubkey : Pubkey::from_str(VOTE_PROGRAM_PUBKEY).unwrap(),
                    is_signer : false,
                    is_writable : false
                },
            ];

            let data = transaction_data::WithdrawData {
                instruction_code : 8,
                padding : [0_u8; 7],
                lamports : amount.map(|sol| (sol * LAMPORTS_PER_SOL) as u64).unwrap_or(0)
            };

            let instruction = Instruction { program_id, accounts, data : AsBytes::as_bytes(&data).to_vec() };

            let tx = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&fee_payer.pubkey()),
                &[&fee_payer, &rewards_authority],
                get_latest_blockhash(&rpc_client)
            );

            submit_transaction(&rpc_client, &tx)
        },
        Command::SetCommission { rewards_authority, commission } => {
            let fee_payer = fee_payer.unwrap();

            let rewards_authority = load_keypair(&rewards_authority, "rewards authority");

            let accounts = vec![
                // Vote Account Manager State Account
                AccountMeta { pubkey : vote_account_manager_state_account, is_signer : false, is_writable : true },
                // Vote Account
                AccountMeta { pubkey : vote_account, is_signer : false, is_writable : true },
                // Rewards Authority
                AccountMeta { pubkey : rewards_authority.pubkey(), is_signer : true, is_writable : false },
                // Vote Program Id
                AccountMeta {
                    pubkey : Pubkey::from_str(VOTE_PROGRAM_PUBKEY).unwrap(),
                    is_signer : false,
                    is_writable : false
                },
            ];

            let data = transaction_data::SetCommissionData { instruction_code : 9, new_commission : commission };

            let instruction = Instruction { program_id, accounts, data : AsBytes::as_bytes(&data).to_vec() };

            let tx = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&fee_payer.pubkey()),
                &[&fee_payer, &rewards_authority],
                get_latest_blockhash(&rpc_client)
            );

            submit_transaction(&rpc_client, &tx)
        },
        Command::Show { json } => {
            match rpc_client.get_account_with_commitment(&vote_account_manager_state_account, commitment) {
                Ok(response) => match response.value {
                    Some(account) => {
                        let data : [u8; 168] = account.data.try_into().unwrap_or_else(|_| {
                            error_exit(&format!("{} is not managed by the Vote Account Manager program", vote_account))
                        });
                        let withdraw_authority = Pubkey::try_from(&data[0..32]).unwrap();
                        let administrator = Pubkey::try_from(&data[32..64]).unwrap();
                        let operational_authority = Pubkey::try_from(&data[64..96]).unwrap();
                        let rewards_authority = Pubkey::try_from(&data[96..128]).unwrap();
                        let commission_data = if data[128] == 0 { None } else { Some((data[129], data[130])) };
                        let leave_epoch = u64::from_le_bytes(data[144..152].try_into().unwrap());
                        if json {
                            print!(
                                "{{\"manager_account_pubkey\":\"{}\",\"withdraw_authority\":\"{}\",\"administrator\":\
                                 \"{}\",\"operational_authority\":\"{}\",\"rewards_authority\":\"{}\"",
                                vote_account_manager_state_account,
                                withdraw_authority,
                                administrator,
                                operational_authority,
                                rewards_authority
                            );
                            match commission_data {
                                Some((max_commission, max_commission_increase_per_epoch)) => {
                                    print!(
                                        ",\"max_commission\":{},\"max_commission_increase_per_epoch\":{}",
                                        max_commission, max_commission_increase_per_epoch
                                    );
                                },
                                None => ()
                            }
                            if leave_epoch > 0 {
                                print!(",\"leave_epoch\":{}", leave_epoch);
                            }
                            println!("}}");
                        }
                        else {
                            println!();
                            println!("Manager Account: {}", vote_account_manager_state_account);
                            println!("Withdraw Authority: {}", withdraw_authority);
                            println!("Administrator: {}", administrator);
                            println!("Operational Authority: {}", operational_authority);
                            println!("Rewards Authority: {}", rewards_authority);
                            match commission_data {
                                Some((max_commission, max_commission_increase_per_epoch)) => {
                                    println!("Max Commission: {}", max_commission);
                                    println!(
                                        "Max Commission Increase per Epoch: {}",
                                        max_commission_increase_per_epoch
                                    );
                                },
                                None => ()
                            }
                            if leave_epoch > 0 {
                                println!("Leave Epoch: {}", leave_epoch);
                            }
                            println!();
                        }
                    },
                    None => error_exit(&format!("{} is not managed by the Vote Account Manager program", vote_account))
                },
                Err(e) => error_exit(&format!("Failed to acquire vote account manager account: {}", e))
            }
        }
    }
}

fn set_authority(
    rpc_client : RpcClient,
    program_id : Pubkey,
    vote_account : Pubkey,
    vote_account_manager_state_account : Pubkey,
    fee_payer : Keypair,
    instruction_code : u8,
    authority : &str,
    authority_name : &str,
    new_authority : &str,
    new_authority_name : &str
)
{
    let authority = load_keypair(authority, authority_name);

    let new_authority = make_pubkey(new_authority, new_authority_name);

    let accounts = vec![
        // Vote Account Manager State Account
        AccountMeta { pubkey : vote_account_manager_state_account, is_signer : false, is_writable : true },
        // Vote Account
        AccountMeta { pubkey : vote_account, is_signer : false, is_writable : false },
        // Authority
        AccountMeta { pubkey : authority.pubkey(), is_signer : true, is_writable : false },
    ];

    let data = transaction_data::SetAuthorityData { instruction_code, new_authority : new_authority.to_bytes() };

    let instruction = Instruction { program_id, accounts, data : AsBytes::as_bytes(&data).to_vec() };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &authority],
        get_latest_blockhash(&rpc_client)
    );

    submit_transaction(&rpc_client, &tx)
}

fn load_keypair_helper(s : &str) -> Result<Keypair, String>
{
    let contents =
        std::fs::read_to_string(s).map_err(|e| format!("Failed to read keypair from {}: {}", s, e.to_string()))?;

    if !contents.starts_with("[") || !contents.ends_with("]") {
        return Err(format!("Malformed keypair file contents in {}", s));
    }

    let contents = &contents[1..(contents.len() - 1)];

    Keypair::from_bytes(u8_list_to_vec(&contents)?.as_slice())
        .map_err(|e| format!("Invalid keypair file contents in {}: {}", s, e))
}

fn load_keypair(
    s : &str,
    desc : &str
) -> Keypair
{
    load_keypair_helper(&s).unwrap_or_else(|e| error_exit(&format!("Failed to load {} keypair: {}", desc, e)))
}

fn u8_list_to_vec(bytes : &str) -> Result<Vec<u8>, String>
{
    bytes
        .replace(" ", "")
        .split(",")
        .map(|s| s.parse::<u8>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<u8>, String>>()
}

fn make_pubkey_helper(s : &str) -> Result<Pubkey, String>
{
    load_keypair_helper(s).map(|keypair| keypair.pubkey()).or_else(|_| Pubkey::from_str(&s).map_err(|e| e.to_string()))
}

fn make_pubkey(
    s : &str,
    desc : &str
) -> Pubkey
{
    make_pubkey_helper(s).unwrap_or_else(|e| error_exit(&format!("Failed to create {} pubkey: {}", desc, e)))
}

fn get_latest_blockhash(rpc_client : &RpcClient) -> Hash
{
    rpc_client
        .get_latest_blockhash()
        .unwrap_or_else(|e| error_exit(&format!("Failed to fetch latest blockhash: {}", e)))
}

fn submit_transaction(
    rpc_client : &RpcClient,
    tx : &Transaction
)
{
    println!(
        "Transaction submitted with signature: {}",
        rpc_client
            .send_and_confirm_transaction_with_spinner(tx)
            .unwrap_or_else(|e| error_exit(&format!("Failed to submit transaction: {}", e)))
    );
}
