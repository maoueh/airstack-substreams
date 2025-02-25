use crate::{
    abi,
    pb::token_tracker::{self},
};
use substreams::{log, Hex};

// pub const ERC721_IFACE_ID: [u8; 4] = <[u8; 4]>::from_hex("01ffc9a7").unwrap();
pub const ERC721_IFACE_ID: [u8; 4] = [0x01, 0xff, 0xc9, 0xa7];
// 0x5b5e139f
pub const ERC721_METADATA_IFACE_ID: [u8; 4] = [0x5b, 0x5e, 0x13, 0x9f];
// 0x780e9d63
pub const ERC721_ENUMERABLE_IFACE_ID: [u8; 4] = [0x78, 0x0e, 0x9d, 0x63];
// pub const ERC1155_IFACE_ID: [u8; 4] = <[u8; 4]>::from_hex("d9b67a26").unwrap();
pub const ERC1155_IFACE_ID: [u8; 4] = [0xd9, 0xb6, 0x7a, 0x26];
// 0x0e89341c
pub const ERC1155_METADATA_URI_IFACE_ID: [u8; 4] = [0x0e, 0x89, 0x34, 0x1c];

// 0x150b7a02
//pub const ERC721_TOKEN_RECEIVER_IFACE_ID: [u8; 4] = [0x01, 0x50, 0xb7, 0x02];

pub fn get_token(
    token_address: String,
    block_number: String,
    block_timestamp: String,
    tx_hash: String,
    from: String,
) -> Option<token_tracker::Token> {
    use abi::erc1155::functions as erc1155_functions;
    use abi::erc20::functions as erc20_functions;
    use abi::erc721::functions as erc721_functions;

    let token_address_bytes = Hex::decode(token_address.clone()).unwrap();

    // erc20 call
    let name_res = erc20_functions::Name {}.call(token_address_bytes.clone());
    let symbol_res = erc20_functions::Symbol {}.call(token_address_bytes.clone());
    let decimals_res = erc20_functions::Decimals {}.call(token_address_bytes.clone());
    let total_supply_res = erc20_functions::TotalSupply {}.call(token_address_bytes.clone());

    if let (Some(name), Some(symbol), Some(decimals), Some(total_supply)) =
        (name_res, symbol_res, decimals_res, total_supply_res) {
        return Some(token_tracker::Token {
            chain_id: 1.to_string(),
            token_address: token_address.clone(),
            token_type: 1,
            deployment_transaction_hash: tx_hash,
            deployment_block: block_number,
            deployment_timestamp: block_timestamp,
            deployer: from,
            name: Some(name),
            symbol: Some(symbol),
            decimals: Some(decimals.as_u64()),
            total_supply: Some(total_supply.to_string()),
            base_uri: None,
            contract_metadata_uri: None,
        });
    }
    // eip1155
    let eip155_iface_resp = erc1155_functions::SupportsInterface {
        interface_id: ERC1155_IFACE_ID,
    }
    .call(token_address_bytes.clone());

    // let eip155_metadata_iface_resp = erc1155_functions::SupportsInterface {
    //     interface_id: ERC1155_METADATA_URI_IFACE_ID,
    // }
    // .call(token_address_bytes.clone());

    if eip155_iface_resp == Some(true)
    // || eip155_metadata_iface_resp == Some(true)
    {
        return Some(token_tracker::Token {
            chain_id: 1.to_string(),
            token_address: token_address.clone(),
            token_type: 3,
            deployment_transaction_hash: tx_hash,
            deployment_block: block_number,
            deployment_timestamp: block_timestamp,
            deployer: from,
            name: None,
            symbol: None,
            decimals: None,
            total_supply: None,
            base_uri: None,
            contract_metadata_uri: None,
        });
    }

    // eip721
    let eip721_iface_resp = erc721_functions::SupportsInterface {
        interface_id: ERC721_IFACE_ID,
    }
    .call(token_address_bytes.clone());

    // let eip721_metadata_iface_resp = erc721_functions::SupportsInterface {
    //     interface_id: ERC721_METADATA_IFACE_ID,
    // }
    // .call(token_address_bytes.clone());

    // let eip721_enumerable_iface_resp = erc721_functions::SupportsInterface {
    //     interface_id: ERC721_ENUMERABLE_IFACE_ID,
    // }
    // .call(token_address_bytes.clone());

    if eip721_iface_resp == Some(true)
    // || eip721_metadata_iface_resp == Some(true)
    // || eip721_enumerable_iface_resp == Some(true)
    {
        let name_res = erc721_functions::Name {}.call(token_address_bytes.clone());
        let symbol_res = erc721_functions::Symbol {}.call(token_address_bytes.clone());
        let total_supply_res = erc721_functions::TotalSupply {}.call(token_address_bytes.clone());

        return Some(token_tracker::Token {
            chain_id: 1.to_string(),
            token_address: token_address.clone(),
            token_type: 2,
            deployment_transaction_hash: tx_hash,
            deployment_block: block_number,
            deployment_timestamp: block_timestamp,
            deployer: from,
            name: name_res,
            symbol: symbol_res,
            decimals: None,
            total_supply: if let Some(total_supply) = total_supply_res {
                Some(total_supply.to_string())
            } else {
                Some(String::from("0"))
            },
            base_uri: None,
            contract_metadata_uri: None,
        });
    }

    None
}
