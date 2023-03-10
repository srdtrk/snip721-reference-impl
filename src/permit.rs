use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NftPermissions {
    /// Allowance for SNIP-20 - Permission to query allowance of the owner & spender
    Allowance,
    /// Balance for SNIP-721 - Permission to query all nft balance of the permit creator
    Balance,
    /// Balances of listed token_ids for SNIP-721 - Permission to query the nft balances of the given token_ids
    BalanceOf(Vec<String>),
    /// History for SNIP-721 - Permission to query transfer_history & transaction_history
    History,
    /// Metadata for SNIP-721 - Permission to query private metadata of all NFTs owned by the permit creator
    Metadata,
    /// Metadata of listed token_ids for SNIP-721 - Permission to query private metadata of the given token_ids
    MetadataOf(Vec<String>),
    /// Owner permission indicates that the bearer of this permit should be granted all
    /// the access of the creator/signer of the permit.  SNIP-721 uses this to grant
    /// viewing access to all data that the permit creator owns and is whitelisted for.
    /// For SNIP-721 use, a permit with Owner permission should NEVER be given to
    /// anyone else.  If someone wants to share private data, they should whitelist
    /// the address they want to share with via a SetWhitelistedApproval tx, and that
    /// address will view the data by creating their own permit with Owner permission
    Owner,
}
