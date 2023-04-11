use crate::error_exit;
use crate::usage;

#[derive(Debug)]
pub enum Command
{
    Enter
    {
        withdraw_authority : String,
        administrator : String,
        max_commission : Option<u8>,
        max_commission_increase_per_epoch : Option<u8>
    },

    SetLeaveEpoch
    {
        withdraw_authority : String, leave_epoch : u64
    },

    Leave
    {
        withdraw_authority : String
    },

    SetAdministrator
    {
        withdraw_authority : String, administrator : String
    },

    SetOperationalAuthority
    {
        administrator : String, authority : String
    },

    SetRewardsAuthority
    {
        administrator : String, authority : String
    },

    SetVoteAuthority
    {
        operational_authority : String, authority : String
    },

    SetValidatorIdentity
    {
        operational_authority : String, validator_identity : String
    },

    Withdraw
    {
        rewards_authority : String, recipient : String, amount : Option<f64>
    },

    SetCommission
    {
        rewards_authority : String, commission : u8
    },

    Show
    {
        json : bool
    }
}

// Returns (Option<fee_payer>, url, Option<commitment>, vote_account, command)
pub fn parse_command() -> (Option<String>, String, Option<String>, String, Command)
{
    let args = parse_args();

    if args.help {
        do_help(args.tokens);
    }

    if args.tokens.len() == 0 {
        error_exit("No command supplied.");
    }

    let command = args.tokens[0].clone();

    let mut tokens = args.tokens.into_iter().skip(1);

    let (maybe_fee_payer, vote_account, command) = match command.as_str() {
        "enter" => {
            let withdraw_authority : String =
                get_arg_value(tokens.next(), args.withdraw_authority, "enter", "withdraw authority");

            (
                Some(withdraw_authority.clone()),
                get_arg_value::<String>(tokens.next(), args.vote_account, "enter", "vote account"),
                Command::Enter {
                    withdraw_authority,
                    administrator : get_arg_value(tokens.next(), args.administrator, "enter", "administrator"),
                    max_commission : get_option_arg_value(tokens.next(), args.max_commission, "max commission"),
                    max_commission_increase_per_epoch : get_option_arg_value(
                        tokens.next(),
                        args.max_commission_increase_per_epoch,
                        "max commission increase per epoch"
                    )
                }
            )
        },
        "set-leave-epoch" => {
            let withdraw_authority : String =
                get_arg_value(tokens.next(), args.withdraw_authority, "set-leave-epoch", "withdraw authority");

            (
                Some(withdraw_authority.clone()),
                get_arg_value::<String>(tokens.next(), args.vote_account, "set-leave-epoch", "vote account"),
                Command::SetLeaveEpoch {
                    withdraw_authority,
                    leave_epoch : get_arg_value(tokens.next(), args.leave_epoch, "set-leave-epoch", "leave epoch")
                }
            )
        },
        "leave" => {
            let withdraw_authority : String =
                get_arg_value(tokens.next(), args.withdraw_authority, "leave", "withdraw authority");

            (
                Some(withdraw_authority.clone()),
                get_arg_value::<String>(tokens.next(), args.vote_account, "leave", "vote account"),
                Command::Leave { withdraw_authority }
            )
        },
        "set-administrator" => {
            let withdraw_authority : String =
                get_arg_value(tokens.next(), args.withdraw_authority, "set-administrator", "withdraw authority");

            (
                Some(withdraw_authority.clone()),
                get_arg_value::<String>(tokens.next(), args.vote_account, "set-administrator", "vote account"),
                Command::SetAdministrator {
                    withdraw_authority,
                    administrator : get_arg_value(
                        tokens.next(),
                        args.administrator,
                        "set-administrator",
                        "administrator"
                    )
                }
            )
        },
        "set-operational-authority" => {
            let administrator : String =
                get_arg_value(tokens.next(), args.administrator, "set-operational-authority", "administrator");

            (
                Some(administrator.clone()),
                get_arg_value::<String>(tokens.next(), args.vote_account, "set-operational-authority", "vote account"),
                Command::SetOperationalAuthority {
                    administrator,
                    authority : get_arg_value(
                        tokens.next(),
                        args.operational_authority,
                        "set-operational-authority",
                        "operational authority"
                    )
                }
            )
        },
        "set-rewards-authority" => {
            let administrator : String =
                get_arg_value(tokens.next(), args.administrator, "set-rewards-authority", "administrator");

            (
                Some(administrator.clone()),
                get_arg_value::<String>(tokens.next(), args.vote_account, "set-rewards-authority", "vote account"),
                Command::SetRewardsAuthority {
                    administrator,
                    authority : get_arg_value(
                        tokens.next(),
                        args.rewards_authority,
                        "set-rewards-authority",
                        "rewards authority"
                    )
                }
            )
        },
        "set-vote-authority" => {
            let operational_authority : String =
                get_arg_value(tokens.next(), args.operational_authority, "set-vote-authority", "operational authority");

            (
                Some(operational_authority.clone()),
                get_arg_value::<String>(tokens.next(), args.vote_account, "set-vote-authority", "vote account"),
                Command::SetVoteAuthority {
                    operational_authority,
                    authority : get_arg_value(
                        tokens.next(),
                        args.vote_authority,
                        "set-vote-authority",
                        "vote authority"
                    )
                }
            )
        },
        "set-validator-identity" => {
            let operational_authority : String = get_arg_value(
                tokens.next(),
                args.operational_authority,
                "set-validator-authority",
                "operational authority"
            );

            (
                Some(operational_authority.clone()),
                get_arg_value::<String>(tokens.next(), args.vote_account, "set-validator-identity", "vote account"),
                Command::SetValidatorIdentity {
                    operational_authority,
                    validator_identity : get_arg_value(
                        tokens.next(),
                        args.validator_identity,
                        "set-validator-identity",
                        "validator identity"
                    )
                }
            )
        },
        "withdraw" => {
            let rewards_authority : String =
                get_arg_value(tokens.next(), args.rewards_authority, "withdraw", "rewards authority");

            (
                Some(rewards_authority.clone()),
                get_arg_value::<String>(tokens.next(), args.vote_account, "withdraw", "vote account"),
                Command::Withdraw {
                    rewards_authority,
                    recipient : get_arg_value(tokens.next(), args.recipient, "withdraw", "recipient"),
                    amount : get_option_arg_value(tokens.next(), args.amount, "amount")
                }
            )
        },
        "set-commission" => {
            let rewards_authority : String =
                get_arg_value(tokens.next(), args.rewards_authority, "set-commission", "rewards authority");

            (
                Some(rewards_authority.clone()),
                get_arg_value::<String>(tokens.next(), args.vote_account, "set-commission", "vote account"),
                Command::SetCommission {
                    rewards_authority,
                    commission : get_arg_value(tokens.next(), args.commission, "set-commission", "commission")
                }
            )
        },
        "show" => {
            (None, get_arg_value::<String>(tokens.next(), args.vote_account, "show", "vote account"), Command::Show {
                json : if args.json.is_some() {
                    true
                }
                else {
                    match tokens.next() {
                        Some(word) => {
                            if word == "json" {
                                true
                            }
                            else {
                                error_exit(&format!("Unexpected argument: {}", word))
                            }
                        },
                        None => false
                    }
                }
            })
        },

        _ => error_exit(&format!("Unknown command: {}", command))
    };

    match tokens.next() {
        Some(token) => error_exit(&format!("Unexpected argument: {}", token)),
        None => ()
    }

    (
        maybe_fee_payer.map(|fee_payer| args.fee_payer.unwrap_or(fee_payer)),
        get_url(args.url),
        args.commitment,
        vote_account,
        command
    )
}

const DEFAULT_MAINNET_RPC_URL : &str = "https://api.mainnet-beta.solana.com";
const DEFAULT_TESTNET_RPC_URL : &str = "https://api.testnet.solana.com";
const DEFAULT_DEVNET_RPC_URL : &str = "https://api.devnet.solana.com";
const DEFAULT_LOCALHOST_RPC_URL : &str = "http://localhost:8899";

#[derive(Default)]
struct Args
{
    fee_payer : Option<String>,

    url : Option<String>,

    commitment : Option<String>,

    vote_account : Option<String>,

    withdraw_authority : Option<String>,

    administrator : Option<String>,

    operational_authority : Option<String>,

    rewards_authority : Option<String>,

    vote_authority : Option<String>,

    max_commission : Option<String>,

    max_commission_increase_per_epoch : Option<String>,

    leave_epoch : Option<String>,

    validator_identity : Option<String>,

    recipient : Option<String>,

    amount : Option<String>,

    commission : Option<String>,

    json : Option<bool>,

    help : bool,

    tokens : Vec<String>
}

fn get_arg(
    index : usize,
    args : &Vec<String>
) -> String
{
    if index == args.len() {
        error_exit(&format!("{} requires an argument", args[index - 1]));
    }

    args[index].clone()
}

fn parse_args() -> Args
{
    let mut args = Args::default();

    let input_args = std::env::args().skip(1).collect::<Vec<String>>();

    let mut i = 0;

    while i < input_args.len() {
        let arg = &input_args[i];
        match arg.as_str() {
            "-f" | "--fee-payer" => {
                if args.fee_payer.is_none() {
                    i += 1;
                    args.fee_payer = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate fee payer");
                }
            },
            "-u" | "--url" => {
                if args.url.is_none() {
                    i += 1;
                    args.url = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate url");
                }
            },
            "-c" | "--commitment" => {
                if args.commitment.is_none() {
                    i += 1;
                    args.commitment = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate commitment");
                }
            },
            "--vote-account" => {
                if args.vote_account.is_none() {
                    i += 1;
                    args.vote_account = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --vote-account");
                }
            },
            "--withdraw-authority" => {
                if args.withdraw_authority.is_none() {
                    i += 1;
                    args.withdraw_authority = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --withdraw-authority");
                }
            },
            "--administrator" => {
                if args.administrator.is_none() {
                    i += 1;
                    args.administrator = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --administrator");
                }
            },
            "--operational-authority" => {
                if args.operational_authority.is_none() {
                    i += 1;
                    args.operational_authority = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --operational-authority");
                }
            },
            "--rewards-authority" => {
                if args.rewards_authority.is_none() {
                    i += 1;
                    args.rewards_authority = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --rewards-authority");
                }
            },
            "--vote-authority" => {
                if args.vote_authority.is_none() {
                    i += 1;
                    args.vote_authority = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --vote-authority");
                }
            },
            "--max-commission" => {
                if args.max_commission.is_none() {
                    i += 1;
                    args.max_commission = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --max-commission");
                }
            },
            "--max-commission-increase-per-epoch" => {
                if args.max_commission_increase_per_epoch.is_none() {
                    i += 1;
                    args.max_commission_increase_per_epoch = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --max-commission-increase-per-epoch");
                }
            },
            "--leave-epoch" => {
                if args.leave_epoch.is_none() {
                    i += 1;
                    args.leave_epoch = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --leave-epoch");
                }
            },
            "--validator-identity" => {
                if args.validator_identity.is_none() {
                    i += 1;
                    args.validator_identity = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --validator-identity");
                }
            },
            "--recipient" => {
                if args.recipient.is_none() {
                    i += 1;
                    args.recipient = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --recipient");
                }
            },
            "--amount" => {
                if args.amount.is_none() {
                    i += 1;
                    args.amount = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --amount");
                }
            },
            "--commission" => {
                if args.amount.is_none() {
                    i += 1;
                    args.commission = Some(get_arg(i, &input_args));
                }
                else {
                    error_exit("Duplicate --commission");
                }
            },
            "--json" => {
                if args.json.is_none() {
                    args.json = Some(true)
                }
                else {
                    error_exit("Duplicate --json");
                }
            },
            "-h" | "--help" | "help" => args.help = true,
            _ => args.tokens.push(arg.clone())
        }
        i += 1;
    }

    args
}

fn get_arg_value<T>(
    token : Option<String>,
    arg : Option<String>,
    command : &str,
    name : &str
) -> T
where
    T : std::str::FromStr
{
    str::parse::<T>(
        &token.xor(arg).unwrap_or_else(|| error_exit(&format!("{} command requires exactly one {}", command, name)))
    )
    .map_err(|_| error_exit(&format!("Invalid value for {}", name)))
    .unwrap()
}

fn get_option_arg_value<T>(
    token : Option<String>,
    arg : Option<String>,
    name : &str
) -> Option<T>
where
    T : std::str::FromStr
{
    token
        .xor(arg)
        .map(|token| str::parse::<T>(&token).map_err(|_| error_exit(&format!("Invalid value for {}", name))).unwrap())
}

fn get_url(url : Option<String>) -> String
{
    url.map_or_else(
        || DEFAULT_MAINNET_RPC_URL.to_string(),
        |url| match url.as_str() {
            "l" | "localhost" => DEFAULT_LOCALHOST_RPC_URL.to_string(),
            "d" | "devnet" => DEFAULT_DEVNET_RPC_URL.to_string(),
            "t" | "testnet" => DEFAULT_TESTNET_RPC_URL.to_string(),
            "m" | "mainnet" => DEFAULT_MAINNET_RPC_URL.to_string(),
            _ => url.clone()
        }
    )
}

fn do_help(tokens : Vec<String>) -> !
{
    let msg = if tokens.len() == 0 {
        usage::USAGE_MESSAGE
    }
    else {
        match tokens[0].as_str() {
            "enter" => usage::ENTER_USAGE_MESSAGE,
            "set-leave-epoch" => usage::SET_LEAVE_EPOCH_USAGE_MESSAGE,
            "leave" => usage::LEAVE_USAGE_MESSAGE,
            "set-administrator" => usage::SET_ADMINISTRATOR_USAGE_MESSAGE,
            "set-operational-authority" => usage::SET_OPERATIONAL_AUTHORITY_USAGE_MESSAGE,
            "set-rewards-authority" => usage::SET_REWARDS_AUTHORITY_USAGE_MESSAGE,
            "set-vote-authority" => usage::SET_VOTE_AUTHORITY_USAGE_MESSAGE,
            "set-validator-identity" => usage::SET_VALIDATOR_IDENTITY_USAGE_MESSAGE,
            "withdraw" => usage::WITHDRAW_USAGE_MESSAGE,
            "set-commission" => usage::SET_COMMISSION_USAGE_MESSAGE,
            "show" => usage::SHOW_USAGE_MESSAGE,
            _ => usage::USAGE_MESSAGE
        }
    };

    println!("{}", msg);

    std::process::exit(0);
}
