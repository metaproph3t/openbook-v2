use anchor_lang::{AccountDeserialize, Discriminator};

use openbook_v2::state::{Market, OpenOrdersAccount, OpenOrdersAccountValue};

use solana_account_decoder::UiAccountEncoding;
use solana_client::nonblocking::rpc_client::RpcClient as RpcClientAsync;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, RpcFilterType};
use solana_sdk::pubkey::Pubkey;

pub async fn fetch_openbook_accounts(
    rpc: &RpcClientAsync,
    program: Pubkey,
    owner: Pubkey,
) -> anyhow::Result<Vec<(Pubkey, OpenOrdersAccountValue)>> {
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![
            RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
                0,
                OpenOrdersAccount::discriminator().to_vec(),
            )),
            RpcFilterType::Memcmp(Memcmp::new_raw_bytes(8, owner.to_bytes().to_vec())),
        ]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            ..RpcAccountInfoConfig::default()
        },
        ..RpcProgramAccountsConfig::default()
    };
    rpc.get_program_accounts_with_config(&program, config)
        .await?
        .into_iter()
        .map(|(key, account)| Ok((key, OpenOrdersAccountValue::from_bytes(&account.data[8..])?)))
        .collect::<Result<Vec<_>, _>>()
}

pub async fn fetch_anchor_account<T: AccountDeserialize>(
    rpc: &RpcClientAsync,
    address: &Pubkey,
) -> anyhow::Result<T> {
    let account = rpc.get_account(address).await?;
    Ok(T::try_deserialize(&mut (&account.data as &[u8]))?)
}

async fn fetch_anchor_accounts<T: AccountDeserialize + Discriminator>(
    rpc: &RpcClientAsync,
    program: Pubkey,
    filters: Vec<RpcFilterType>,
) -> anyhow::Result<Vec<(Pubkey, T)>> {
    let account_type_filter =
        RpcFilterType::Memcmp(Memcmp::new_raw_bytes(0, T::discriminator().to_vec()));
    let config = RpcProgramAccountsConfig {
        filters: Some([vec![account_type_filter], filters].concat()),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            ..RpcAccountInfoConfig::default()
        },
        ..RpcProgramAccountsConfig::default()
    };
    rpc.get_program_accounts_with_config(&program, config)
        .await?
        .into_iter()
        .map(|(key, account)| Ok((key, T::try_deserialize(&mut (&account.data as &[u8]))?)))
        .collect()
}

pub async fn fetch_markets(
    rpc: &RpcClientAsync,
    program: Pubkey,
    group: Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Market)>> {
    fetch_anchor_accounts::<Market>(
        rpc,
        program,
        vec![RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
            8,
            group.to_bytes().to_vec(),
        ))],
    )
    .await
}
