use schemars::JsonSchema;
use secret_toolkit::permit::Permit;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NftPermissions {
    /// Balance for SNIP-721 - Permission to query all nft balance of the permit creator
    ViewOwner,
    /// Balances of listed token_ids for SNIP-721
    /// Permission to query the nft balances of the given token_ids as the permit creator
    ViewOwnerOf(Vec<String>),
    /// History for SNIP-721 - Permission to query transfer_history & transaction_history
    History,
    /// Metadata for SNIP-721 - Permission to query private metadata of all NFTs owned by the permit creator
    Metadata,
    /// Metadata of listed token_ids for SNIP-721
    /// Permission to query private metadata of the given token_ids as the permit creator
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

pub fn check_view_owner_restriction(permit: &Permit<NftPermissions>) -> Option<Vec<String>> {
    let permit_permissions: &Vec<NftPermissions> = &permit.params.permissions;
    let mut result = vec![];
    for permission in permit_permissions {
        match permission {
            NftPermissions::ViewOwner => {
                return None;
            }
            NftPermissions::ViewOwnerOf(vec) => {
                result.append(&mut vec.clone());
            }
            NftPermissions::History => {}
            NftPermissions::Metadata => {}
            NftPermissions::MetadataOf(_) => {}
            NftPermissions::Owner => return None,
        }
    }
    Some(result)
}

pub fn check_nft_permission(permit: &Permit<NftPermissions>, permission: &NftPermissions) -> bool {
    let permit_permissions: &Vec<NftPermissions> = &permit.params.permissions;

    match permission {
        NftPermissions::ViewOwnerOf(token_ids) => {
            return permit_permissions.iter().any(|permit_permission| {
                if let NftPermissions::ViewOwnerOf(permitted_ids) = permit_permission {
                    token_ids.iter().all(|id| permitted_ids.contains(id))
                } else {
                    (permit_permission == &NftPermissions::ViewOwner)
                        || (permit_permission == &NftPermissions::Owner)
                }
            });
        }
        NftPermissions::MetadataOf(token_ids) => {
            return permit_permissions.iter().any(|permit_permission| {
                if let NftPermissions::MetadataOf(permitted_ids) = permit_permission {
                    token_ids.iter().all(|id| permitted_ids.contains(id))
                } else {
                    (permit_permission == &NftPermissions::Metadata)
                        || (permit_permission == &NftPermissions::Owner)
                }
            });
        }
        _ => permit.params.permissions.iter().any(|permit_permission| {
            (permit_permission == permission) || (permit_permission == &NftPermissions::Owner)
        }),
    }
}
