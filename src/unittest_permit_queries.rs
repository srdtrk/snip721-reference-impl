mod tests {
    use std::any::Any;

    use cosmwasm_std::testing::*;
    use cosmwasm_std::{
        from_binary, Addr, Binary, BlockInfo, Env, OwnedDeps, Response, StdError, StdResult,
        Timestamp,
    };

    use crate::contract::{execute, instantiate, query};
    use crate::expiration::Expiration;
    use crate::mint_run::MintRunInfo;
    use crate::msg::{
        AccessLevel, BatchNftDossierElement, Cw721Approval, ExecuteMsg, InstantiateConfig,
        InstantiateMsg, Mint, QueryAnswer, QueryMsg, Snip721Approval, Tx, TxAction, ViewerInfo,
    };
    use crate::token::{Extension, Metadata};

    // Helper functions

    fn init_helper_default() -> (
        StdResult<Response>,
        OwnedDeps<MockStorage, MockApi, MockQuerier>,
    ) {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("instantiator", &[]);
        let init_msg = InstantiateMsg {
            name: "sec721".to_string(),
            symbol: "S721".to_string(),
            admin: Some("admin".to_string()),
            entropy: "We're going to need a bigger boat".to_string(),
            royalty_info: None,
            config: None,
            post_init_callback: None,
        };

        (instantiate(deps.as_mut(), env, info, init_msg), deps)
    }

    fn init_helper_with_config(
        public_token_supply: bool,
        public_owner: bool,
        enable_sealed_metadata: bool,
        unwrapped_metadata_is_private: bool,
        minter_may_update_metadata: bool,
        owner_may_update_metadata: bool,
        enable_burn: bool,
    ) -> (
        StdResult<Response>,
        OwnedDeps<MockStorage, MockApi, MockQuerier>,
    ) {
        let mut deps = mock_dependencies();

        let env = mock_env();
        let init_config: InstantiateConfig = from_binary(&Binary::from(
            format!(
                "{{\"public_token_supply\":{},
          \"public_owner\":{},
          \"enable_sealed_metadata\":{},
          \"unwrapped_metadata_is_private\":{},
          \"minter_may_update_metadata\":{},
          \"owner_may_update_metadata\":{},
          \"enable_burn\":{}}}",
                public_token_supply,
                public_owner,
                enable_sealed_metadata,
                unwrapped_metadata_is_private,
                minter_may_update_metadata,
                owner_may_update_metadata,
                enable_burn,
            )
            .as_bytes(),
        ))
        .unwrap();
        let info = mock_info("instantiator", &[]);
        let init_msg = InstantiateMsg {
            name: "sec721".to_string(),
            symbol: "S721".to_string(),
            admin: Some("admin".to_string()),
            entropy: "We're going to need a bigger boat".to_string(),
            royalty_info: None,
            config: Some(init_config),
            post_init_callback: None,
        };

        (instantiate(deps.as_mut(), env, info, init_msg), deps)
    }

    fn extract_error_msg<T: Any>(error: StdResult<T>) -> String {
        match error {
            Ok(_response) => panic!("Expected error, but had Ok response"),
            Err(err) => match err {
                StdError::GenericErr { msg, .. } => msg,
                _ => panic!("Unexpected error result {:?}", err),
            },
        }
    }
}
