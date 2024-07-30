// src/multicall.rs

use ethers::{
    prelude::{Provider, Http},
    types::{Address, U256},
    prelude::abigen,
};
use ethers_contract::Multicall;
// use ethers_core::abi::Abi;
use eyre::Result;
use std::sync::Arc;


abigen!(
    IUniswapV2Factory,
    r#"[
    function allPairs(uint256 index) external view returns (address pair)
    function allPairsLength() external view returns (uint256)
    ]"#,
);

// Function to perform multicall
pub async fn fetch_pairs(
    provider: Arc<Provider<Http>>,
    factory_address: Address,
    batch_size: usize,
    max_pools: u64,
) -> Result<Vec<Address>> {
    // Initialize the contract instance
    let uniswap_v2_factory = IUniswapV2Factory::new(factory_address, provider.clone());

    // Prepare multicall
    let mut multicall = Multicall::new(provider.clone(), None).await?;

    // Store results
    let mut pairs = Vec::new();

    // Fetch the number of pools
    let num_pools: u64 = uniswap_v2_factory.all_pairs_length().call().await?.as_u64();
    let num_pools = num_pools.min(max_pools);

    // Fetch pairs in batches
    for i in (0..num_pools).step_by(batch_size) {
        let mut calls = Vec::new();
        let end_index = (i + batch_size as u64).min(num_pools);
        for j in i..end_index {
            let index: U256 = U256::from(j);
            let call = uniswap_v2_factory.all_pairs(index);
            calls.push(call);
        }

        // Add calls to multicall
        for call in calls {
            multicall.add_call(call, false);
        }

        // Execute multicall and get results
        let results: Vec<(Address,)> = multicall.call().await?;
        for (pair,) in results {
            pairs.push(pair);
        }
        
        // Clear calls for the next batch
        multicall.clear_calls();
    }

    Ok(pairs)
}
