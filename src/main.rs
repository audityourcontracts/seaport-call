use alloy::{
    dyn_abi::{parser::Parameters, SolType}, eips::BlockId, network::EthereumWallet, node_bindings::Anvil, 
    primitives::{Uint, U256, b256, Address, FixedBytes, Bytes}, 
    providers::{Provider, ProviderBuilder}, 
    rpc::types::Filter, signers::local::PrivateKeySigner, sol, sol_types::{SolCall, SolValue}
};

use url::Url;
use Seaport_1_6::{BasicOrderParameters, OfferItem, Order, OrderComponents, OrderParameters, ConsiderationItem};
use std::env;

sol!(
    #[sol(rpc)]
    "src/abi/Seaport_1_6.sol"
);

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc = env::var("ETH_RPC_URL").map_err(|e| format!("Failed to get ETH_RPC_URL: {}", e))?;
    let rpc_url = Url::parse(&rpc).map_err(|e| format!("Failed to parse RPC URL: {}", e))?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let seaport_contract_address =  Address::parse_checksummed("0x0000000000000068F116a894984e2DB1123eB395", None).expect("Invalid address provided");

    let seaport_instance =  Seaport_1_6::new(seaport_contract_address, &provider);

    let offer_items = vec![
        OfferItem {
            itemType: 0,
            token: Address::parse_checksummed("0x0000000000000000000000000000000000000000", None).expect("Invalid address provided"),
            identifierOrCriteria: Uint::<256, 4>::from(0),
            startAmount: Uint::<256, 4>::from(1),
            endAmount: Uint::<256, 4>::from(1),
        }
    ];

    let consideration_items = vec![
        ConsiderationItem {
            itemType: 0,
            token: Address::parse_checksummed("0x0000000000000000000000000000000000000001", None).expect("Invalid address provided"),
            identifierOrCriteria: Uint::<256, 4>::from(0),
            startAmount: Uint::<256, 4>::from(1),
            endAmount: Uint::<256, 4>::from(1),
            recipient: Address::parse_checksummed("0x0000000000000000000000000000000000000002", None).expect("Invalid address provided"),
        }
    ];

    let basic_order_params = Order{
        parameters: OrderParameters{
            offerer: Address::parse_checksummed("0x0000000000000000000000000000000000000003", None).expect("Couldn't parse"),
            zone: Address::parse_checksummed("0x0000000000000000000000000000000000000004", None).expect("Couldn't parse"), 
            orderType: 0,
            offer: offer_items,
            consideration: consideration_items,
            startTime: Uint::<256, 4>::from(1000),
            endTime: Uint::<256, 4>::from(1000),
            zoneHash: FixedBytes::<32>::from([0u8; 32]),
            salt: Uint::<256, 4>::from(1),
            conduitKey: FixedBytes::<32>::from([0u8; 32]),
            totalOriginalConsiderationItems: Uint::<256, 4>::from(1000),
        },
        signature: Bytes::from(vec![0u8; 1]),
    };

    let process_order = seaport_instance.fulfillOrder(basic_order_params, FixedBytes::<32>::from([0u8; 32]));

    println!("FulfillOrder Calldata: {}", process_order.calldata());

    Ok(())
}
