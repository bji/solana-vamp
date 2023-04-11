pub const USAGE_MESSAGE : &str = "
solana-vamp is a utility program that can be used to interact with the Solana
Vote Account Manager program.

Usage:
  solana-vamp enter                      -- To start using VAMP
  solana-vamp set-leave-epoch            -- To set a leave epoch
  solana-vamp leave                      -- To stop using SOLANA-VAMP
  solana-vamp set-administrator          -- To set the administrator
  solana-vamp set-operational-authority  -- To set the operational authority
  solana-vamp set-rewards-authority      -- To set the rewards authority
  solana-vamp set-vote-authority         -- To set the vote authority
  solana-vamp set-validator-identity     -- To set the validator identity
  solana-vamp withdraw                   -- To withdraw from a vote account
  solana-vamp set-commission             -- To set commission
  solana-vamp show                       -- To show managed state
  solana-vamp help                       -- To print this help message

For help on a specific command, use 'solana-vamp help <COMMAND>', for example:

$ solana-vamp help enter
";

pub const ENTER_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp enter
              --withdraw-authority <WITHDRAW_AUTHORITY_FILE>
              --vote-acount <VOTE_ACCOUNT>
              --administrator <ADMINISTRATOR>
              [--max-commission <MAX_COMMISSION>
               --max-commission-increase-per-epoch <MAX_INCREASE_PER_EPOCH>]
              [--fee-payer <FEE_PAYER>]
              [--url <RPC_ENDPOINT>]

'solana-vamp enter' initiates the use of the Vote Account Manager program with
a particular vote account.  This will bring the vote account under the control
of the Vote Account Manager program.  While under control of the program,
actions taken on the vote account must all be issued using the solana-vamp
command (except for set-vote-authority, which can be accomplished by the vote
authority as well as by solana-vamp).

The following arguments are required:

  --withdraw-authority: Must be the path to the withdraw authority keypair of
      the vote account at the time that the 'solana-vamp enter' command is
      executed.  The vote account will have its withdraw authority replaced by
      an authority controlled by the program; but the original
      --withdraw-authority must be retained by the user as it will be used for
      high level authentication by the vamp program.

  --vote-account: Must be the pubkey of the vote account which will be put
      under control of the Vote Account Manager program, or the path to a
      keypair file from which the vote account pubkey will be loaded.

  --administrator: Must be the pubkey of the initial administrator of the Vote
      Account Manager managed account, or the path to a keypair file from
      which the administrator pubkey will be loaded.  The --administrator will
      initially have authority for all actions except set-leave-epoch, leave,
      and set-administrator.

The following arguments may be optionally provided:

  --max-commission: If supplied, the Vote Account Manager will enforce a
      maximum commission on the vote account.  It will not allow any
      commission to be set that is larger than this --max-commission value.
      If this value is not supplied, then the Vote Account Manager program
      will not limit the commission which may be set on the vote account.
      NOTE that if this value is set, then the vote account cannot leave
      control by the program until a leave epoch has been set, and that leave
      epoch has been reached.

  --max-commission-increase-per-epoch: Must be suppled if --max-commission is
      supplied.  This gives the maximum increase, in absolute value terms,
      that is allowed for the vote account per epoch.

  --fee-payer: Will set the fee payer for the transaction to the keypair
      stored in the given file.  If this argument is not present, the
      --withdraw-authority will be used as the fee payer.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

Examples:

# Put vote account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz under control
# of the Vote Account Manager program.  The withdraw authority of the vote
# account is the keypair at withdraw_authority.json.  The initial
# administrator is the keypair at administrator.json.

$ solana-vamp enter                                                           \\
      --withdraw-authority withdraw_authority.json                            \\
      --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz             \\
      --administrator administrator.json

# Put the same vote account under control of the Vote Account Manager program,
# but use a specific fee payer, and also set a maximum commission of 10% and a
# maximum commission change per epoch of 3%.

$ solana-vamp enter                                                           \\
      --fee-payer fee_payer.json                                              \\
      --withdraw-authority withdraw_authority.json                            \\
      --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz             \\
      --administrator administrator.json                                      \\
      --max-commission 10                                                     \\
      --max-commission-increase-per-epoch 3
";

pub const SET_LEAVE_EPOCH_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp set-leave-epoch
              --withdraw-authority <WITHDRAW_AUTHORITY_FILE>
              --vote-acount <VOTE_ACCOUNT>
              --leave-epoch <EPOCH>
              [--fee-payer <FEE_PAYER>]
              [--url <RPC_ENDPOINT>]

'solana-vamp set-leave-epoch' sets the earliest epoch at which a 'leave'
command may be issued to return control of the vote account to the withdraw
authority.

The following arguments are required:

  --withdraw-authority: Must be the path to the original withdraw authority
      keypair of the vote account.  Only this keypair retains authority to set
      the leave epoch after the vote account has been placed under program
      control.

  --vote-account: Must be the pubkey of the vote account under program
      control, or the path to a keypair file from which the vote account
      pubkey will be loaded.

  --leave-epoch: Must be the number of the epoch which is set as the earliest
      epoch that the vote account may leave program control.  Note that this
      value must be at least the current epoch + 2.

The following arguments may be optionally provided:

  --fee-payer: Will set the fee payer for the transaction to the keypair
      stored in the given file.  If this argument is not present, the
      --withdraw-authority will be used as the fee payer.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

set-leave-epoch is only necessary if the Vote Account Manager program has been
configured to restrict commission on the vote account.  If this is the case,
then the Vote Account Manage program requires that a \"leave epoch\" be set
before allowing the leave command to take control of the vote account away
from the program.  This prevents the withdraw authority from getting around
the max commission values by removing the vote account from program control
and then setting the commission directly.

The \"leave epoch\" must be at least 1 full epoch beyond the current epoch.
This in effect puts a minimum amount of time before the withdraw authority
must wait to leave the program at 1 full epoch.  Thus any stakers of the vote
account will have at least 1 full epoch to decide whether to de-stake based on
the validator's intention to leave the program and thus revoke the controls
that have been placed on its commission.

For example, if the current epoch is 100, then the earliest leave epoch that
may be set is 102.  During epoch 101, stakers may decide how to respond to the
validator's intention to stop using the program.

Note that after the leave command is successfully issued, no commission
changes will be allowed for the validator.  The validator operator must remove
the validator from the program and then can change commission at will, or can
put the vote account back under the program's control with a new 'enter'
command, setting new commission limits if desired.

Example:

# Request to be able to remove vote account
# 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz from the program no earlier
# than epoch 120.  The original withdraw authority of the vote account is
# provided in the keypair file withdraw_authority.json.

$ solana-vamp set-leave-epoch                                                 \\
      --withdraw_authority withdraw_authority.json                            \\
      --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz             \\
      --leave-epoch 120
";

pub const LEAVE_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp leave
              --withdraw-authority <WITHDRAW_AUTHORITY_FILE>
              --vote-acount <VOTE_ACCOUNT>
              [--fee-payer <FEE_PAYER>]
              [--url <RPC_ENDPOINT>]

'solana-vamp leave' removes the vote account from Vote Account Manager program
control, and re-sets the vote account's withdraw authority to
<WITHDRAW_AUTHORITY>.  Note that if the program is enforcing commission limits
on the vote account, then a leave epoch must have been set by the 'solana-vamp
set-leave-epoch' command, and the current epoch must be at or beyond that
leave epoch, for 'solana-vamp leave' to succeed.

The following arguments are required:

  --withdraw-authority: Must be the keypair of the original withdraw authority
      of the vote account.  Only this keypair retains authority to leave the
      program after the vote account has been placed under program control.

  --vote-account: Must be the pubkey of the vote account under program
      control, or the path to a keypair file from which the vote account
      pubkey will be loaded.

The following arguments may be optionally provided:

  --fee-payer: Will set the fee payer for the transaction to the keypair
      stored in the given file.  If this argument is not present, the
      --withdraw-authority will be used as the fee payer.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

Example:

# Remove the vote account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz from
# program control.  The original vote account withdraw authority is provided
# in the keyfile withdraw_authority.json.

$ solana-vamp leave                                                           \\
      --withdraw-authority withdraw_authority.json                            \\
      --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz
";

pub const SET_ADMINISTRATOR_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp set-administrator
              --withdraw-authority <WITHDRAW_AUTHORITY_FILE>
              --vote-account <VOTE_ACCOUNT>
              --administrator <NEW_ADMINISTRATOR>
              [--fee-payer <FEE_PAYER>]
              [--url <RPC_ENDPOINT>]

'solana-vamp set-administrator' sets the pubkey of the new administrator
account to be used to authenticate these commands:
    set-operational-authority
    set-rewards-authority

The following arguments are required:

  --withdraw-authority: Must be the keypair of the original withdraw authority
      of the vote account.  Only this keypair retains authority to leave the
      program after the vote account has been placed under program control.

  --vote-account: Must be the pubkey of the vote account under program
      control, or the path to a keypair file from which the vote account
      pubkey will be loaded.

  --administrator: Must be the pubkey of the new administrator, or the path of
      the keypair file from which the new administrator pubkey will be read.

The following arguments may be optionally provided:

  --fee-payer: Will set the fee payer for the transaction to the keypair
      stored in the given file.  If this argument is not present, the
      --administrator will be used as the fee payer.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

Example:

# Set the administrator for vote account
# 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz to new_administrator.json.  The
# original vote account withdraw authority is provided in the keyfile
# withdraw_authority.json.

$ solana-vamp set-administrator                                               \\
      --withdraw-authority withdraw_authority.json                            \\
      --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz             \\
      --administrator new_administrator.json
";

pub const SET_OPERATIONAL_AUTHORITY_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp set-operational-authority
              --administrator <ADMINISTRATOR_FILE>
              --vote-account <VOTE_ACCOUNT>
              --operational-authority <OPERATIONAL_AUTHORITY>
              [--fee-payer <FEE_PAYER>]
              [--url <RPC_ENDPOINT>]

'solana-vamp set-operational-authority' sets the pubkey of the new operational
authority account to be used to authenticate these commands:
    set-vote-authority
    set-validator-identity

The following arguments are required:

  --administrator: Must be the keypair of the administrator of the vote
      account.  Only this keypair retains authority to change the operational
      authority for the vote account.

  --vote-account: Must be the pubkey of the vote account under program
      control, or the path to a keypair file from which the vote account
      pubkey will be loaded.

  --operational-authority: Must be the pubkey of the new operational
      authority, or the path of the keypair file from which the new
      operational authority pubkey will be read.

The following arguments may be optionally provided:

  --fee-payer: Will set the fee payer for the transaction to the keypair
      stored in the given file.  If this argument is not present, the
      --administrator will be used as the fee payer.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

Example:

# Set the operational authority for vote account
# 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz to
# new_operational_authority.json.  The administrator is provided in the
# keyfile administrator.json.

$ solana-vamp set-operational-authority                                       \\
      --administrator administrator.json                                      \\
      --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz             \\
      --operational-authority new_operational_authority.json
";

pub const SET_REWARDS_AUTHORITY_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp set-rewards-authority
              --administrator <ADMINISTRATOR_FILE>
              --vote-account <VOTE_ACCOUNT>
              --rewards-authority <REWARDS_AUTHORITY>
              [--fee-payer <FEE_PAYER>]
              [--url <RPC_ENDPOINT>]

'solana-vamp set-rewards-authority' sets the pubkey of the new rewards
authority account to be used to authenticate these commands:
    withdraw
    set-commission

The following arguments are required:

  --administrator: Must be the keypair of the administrator of the vote
      account.  Only this keypair retains authority to change the rewards
      authority for the vote account.

  --vote-account: Must be the pubkey of the vote account under program
      control, or the path to a keypair file from which the vote account
      pubkey will be loaded.

  --rewards-authority: Must be the pubkey of the new rewards authority, or the
      path of the keypair file from which the new rewards authority pubkey
      will be read.

The following arguments may be optionally provided:

  --fee-payer: Will set the fee payer for the transaction to the keypair
      stored in the given file.  If this argument is not present, the
      --administrator will be used as the fee payer.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

Example:

# Set the rewards authority for vote account
# 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz to new_rewards_authority.json.
# The administrator is provided in the keyfile administrator.json.

$ solana-vamp set-rewards-authority                                           \\
      --administrator administrator.json                                      \\
      --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz             \\
      --rewards-authority new_rewards_authority.json
";

pub const SET_VOTE_AUTHORITY_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp set-vote-authority
              --operational-authority <OPERATIONAL_AUTHORITY_FILE>
              --vote-account <VOTE_ACCOUNT>
              --vote-authority <NEW_VOTE_AUTHORITY>
              [--fee-payer <FEE_PAYER>]
              [--url <RPC_ENDPOINT>]

'solana-vamp set-vote-authority' sets the pubkey of the new vote authority
account for the vote account.  This can also be accomplished by the normal
'solana vote-authorize-voter' command, since the vote authority of the vote
account (which the Vote Account Manager does not change) always has rights to
switch to a new vote authority.

The following arguments are required:

  --operational-authority: Must be the keypair of the operational authority of
      the vote account.  Only this keypair retains authority to change the
      vote authority for the vote account via the solana-vamp program.

  --vote-account: Must be the pubkey of the vote account under program
      control, or the path to a keypair file from which the vote account
      pubkey will be loaded.

  --vote-authority: Must be the pubkey of the new vote authority, or the path
      of the keypair file from which the new vote authority pubkey will be
      read.

The following arguments may be optionally provided:

  --fee-payer: Will set the fee payer for the transaction to the keypair
      stored in the given file.  If this argument is not present, the
      --operational-authority will be used as the fee payer.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

Example:

# Set the vote authority for vote account
# 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz to new_vote_authority.json.
# The operational authority is provided in the keyfile
# operational_authority.json.

$ solana-vamp set-vote-authority                                              \\
      --operational-authority operational_authority.json                      \\
      --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz             \\
      --vote-authority new_vote_authority.json
";

pub const SET_VALIDATOR_IDENTITY_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp set-validator-identity
              --operational-authority <OPERATIONAL_AUTHORITY_FILE>
              --vote-account <VOTE_ACCOUNT>
              --validator-identity <NEW_VALIDATOR_IDENTITY_FILE>
              [--fee-payer <FEE_PAYER>]
              [--url <RPC_ENDPOINT>]

'solana-vamp set-validator-identity' sets the keypair of the new validator
identity for the vote account.

The following arguments are required:

  --operational-authority: Must be the keypair of the operational authority of
      the vote account.  Only this keypair retains authority to change the
      validator identity for the vote account.

  --vote-account: Must be the pubkey of the vote account under program
      control, or the path to a keypair file from which the vote account
      pubkey will be loaded.

  --validator-identity: Must be the path of the keypair file from which the
      new validator identity will be read.

The following arguments may be optionally provided:

  --fee-payer: Will set the fee payer for the transaction to the keypair
      stored in the given file.  If this argument is not present, the
      --operational-authority will be used as the fee payer.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

Example:

# Set the validator identity for vote account
# 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz to new_validator_identity.json.
# The operational authority is provided in the keyfile
# operational_authority.json.

$ solana-vamp set-validator-identity                                          \\
      --operational-authority operational_authority.json                      \\
      --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz             \\
      --validator-identity new_validator_identity.json
";

pub const WITHDRAW_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp withdraw
              --rewards-authority <REWARDS_AUTHORITY>
              --vote-account <VOTE_ACCOUNT>
              --recipient <RECIPIENT_ACCOUNT>
              [--amount <SOL_TO_WITHDRAW>]
              [--fee-payer <FEE_PAYER>]
              [--url <RPC_ENDPOINT>]

'solana-vamp withdraw' withdraws SOL from the vote account.  It will never
withdraw below the rent exempt reserve of the vote account.

The following arguments are required:

  --rewards-authority: Must be the keypair of the rewards authority of the
      vote account.  Only this keypair retains authority to withdraw from the
      vote account.

  --vote-account: Must be the pubkey of the vote account under program
      control, or the path to a keypair file from which the vote account
      pubkey will be loaded.

  --recipient: Must be the pubkey of the account into which the SOL will be
      withdrawn, or the path to a keypair file from which the recipient pubkey
      will be read.

The following arguments may be optionally provided:

  --amount: The quantity of SOL to withdraw from the vote account.  If this
      value is not present, or is specified as '0', then the maximum amount of
      SOL that can be withdrawn from the vote account while respecting rent
      exempt minimums will be withdrawn.

  --fee-payer: Will set the fee payer for the transaction to the keypair
      stored in the given file.  If this argument is not present, the
      --rewards-authority will be used as the fee payer.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

Examples:

# Withdraw 10.05 SOL from vote account
# 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz into user_key.json.  The
# rewards authority is provided in the keyfile rewards_authority.json.

$ solana-vamp withdraw                                                        \\
              --rewards-authority rewards_authority.json                      \\
              --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz     \\
              --recipient user_key.json                                       \\
              --amount 10.05

# Withdraw all available funds from the vote account:

$ solana-vamp withdraw                                                        \\
              --rewards-authority rewards_authority.json                      \\
              --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz     \\
              --recipient user_key.json
";

pub const SET_COMMISSION_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp set-commission
              --rewards-authority <REWARDS_AUTHORITY>
              --vote-account <VOTE_ACCOUNT>
              --commission <NEW_COMMISSION>
              [--fee-payer <FEE_PAYER>]
              [--url <RPC_ENDPOINT>]

'solana-vamp set-commission' sets the vote account's commission to a new
value.  If the Vote Account Manager program has been configured to enforce
commission caps on the vote account, then NEW_COMMISSION must not violate
those caps.

The following arguments are required:

  --rewards-authority: Must be the keypair of the rewards authority of the
      vote account.  Only this keypair retains authority to change commission
      of the vote account.

  --vote-account: Must be the pubkey of the vote account under program
      control, or the path to a keypair file from which the vote account
      pubkey will be loaded.

  --commission: The new commission to set.  If commission caps are in effect
      for the vote account, then NEW_COMMISSION must be no greater than the
      max allowed commission, and must be no more than the maximum commission
      increase above the commission that the vote account was set to when the
      current epoch begin.  Also, if a leave epoch has been set, then the
      commission cannot be changed.

The following arguments may be optionally provided:

  --fee-payer: Will set the fee payer for the transaction to the keypair
      stored in the given file.  If this argument is not present, the
      --rewards-authority will be used as the fee payer.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

Example:

# Set vote account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz to have
# commission 5%.  The rewards authority is provided in rewards_authority.json

$ solana-vamp set-commission                                                  \\
              --rewards-authority rewards_authority.json                      \\
              --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz     \\
              --commission 5
";

pub const SHOW_USAGE_MESSAGE : &str = "
Usage:
  solana-vamp show
              --vote-account <VOTE_ACCOUNT>
              [--json]
              [--url <RPC_ENDPOINT>]

'solana-vamp show' shows the currently configured values for a vote account
under control of the Vote Account Manager program.

The following arguments is required:

  --vote-account: Must be the pubkey of the vote account under program
      control, or the path to a keypair file from which the vote account
      pubkey will be loaded.

The following arguments may be optionally provided:

  --json: The output format will be JSON; if this argument is not provided, it
      will be human readable lines.

  --url: Will set the URL of the RPC endpoint to send transactions to.  A full
      URL may be specified, and in addition, the following special values may
      be used:
        l, localhost: http://localhost:8899
        d, devnet: https://api.devnet.solana.com
        t, testnet: https://api.testnet.solana.com
        m, mainnet: https://api.mainnet-beta.solana.com
      If --url is not supplied, then mainnet is used.

Example:

# Show the configuration values for the vote account
# 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz

$ solana-vamp show                                                            \\
              --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz

Manager Account: ABsS4JPCWYyN1evPJpudm7apmEZp5NTocN3CAxKnSCQk
Withdraw Authority: 3cnbBcMULnSoyLtgGNwrEPdLiqwuzpU4bVpro2m71vn2
Administrator: 3wHoK6DTF9jPCqDQgp99RF88qo4QPyKca9gxxSMHYsMu
Operational Authority: B2YVSHfY3uK5egSzvt1unMchmdo3mxiC2grMxQpxf7DB
Rewards Authority: DchTjdEyR8ea46ofauxnVPMRZvBnCpkYkYixSXpQfNnk
Max Commission: 10
Max Commission Increase Per Epoch: 3

# Show again, this time, in JSON format, using jq to pretty-print the JSON:

$ solana-vamp show                                                            \\
              --vote-account 3yP1VFUXzgND1UoLiVeu5AST46Ze6FVnR4DH7DDrgYTz     \\
              --json                                                          \\
  | jq .

{
  \"manager_account_pubkey\": \"ABsS4JPCWYyN1evPJpudm7apmEZp5NTocN3CAxKnSCQk\",
  \"withdraw_authority\": \"3cnbBcMULnSoyLtgGNwrEPdLiqwuzpU4bVpro2m71vn2\",
  \"administrator\": \"3wHoK6DTF9jPCqDQgp99RF88qo4QPyKca9gxxSMHYsMu\",
  \"operational_authority\": \"B2YVSHfY3uK5egSzvt1unMchmdo3mxiC2grMxQpxf7DB\",
  \"rewards_authority\": \"DchTjdEyR8ea46ofauxnVPMRZvBnCpkYkYixSXpQfNnk\",
  \"max_commission\": 10,
  \"max_commission_increase_per_epoch\": 3
}
";
