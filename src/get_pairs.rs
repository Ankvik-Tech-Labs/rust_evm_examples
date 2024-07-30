use ethers::{
    prelude::abigen,
    providers::{Provider},
    types::{Address, U256},
    // solc::Solc,
};
use std::sync::Arc;
use std::fs;
use std::time::Instant;
use toml::Value;
use eyre::Result;
use colored::*;

abigen!(
    IUniswapV2Factory,
    r#"[
    function allPairs(uint256 index) external view returns (address pair)
    function allPairsLength() external view returns (uint256)
    ]"#,
);


fn load_rpc_url(config_path: &str) -> Result<String> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Value = toml::from_str(&config_content)?;
    let web3_endpoints = config
        .get("rpc")
        .and_then(|rpc| rpc.get("web3_endpoints"))
        .and_then(|endpoints| endpoints.as_array())
        .and_then(|arr| arr.get(1)) // Load the 2nd RPC url
        .and_then(|url| url.as_str())
        .ok_or_else(|| eyre::eyre!("Failed to load RPC URL from config"))?;
    Ok(web3_endpoints.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now(); // Start timing
    let rpc_url = load_rpc_url("config.toml")?;
    // println!("{}", rpc_url);
    let provider = Arc::new(Provider::try_from(rpc_url)?);

    // Initialize a new instance of UNISWAP_V2_Factory
    let pair_address: Address = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse()?;
    let uniswap_v2_factory = IUniswapV2Factory::new(pair_address, provider);

    
    let num_pools: U256 = uniswap_v2_factory.all_pairs_length().call().await?;
    println!("{} {}", "[*] Number of pools:".truecolor(255, 0, 212), num_pools.to_string().cyan());
    
    let mut pairs = Vec::new();
    // for testing limit to first 100 pools
    for i in 0..num_pools.as_u64().min(100) {
        let index: U256 = U256::from(i);
        // Use the allPairs() function to fetch the pool reserves
        let pair = uniswap_v2_factory.all_pairs(index).call().await?;
        pairs.push(pair);
    }
    println!("{} {:?}", "[*] Pair addresses:".green(), pairs);

    let duration = start_time.elapsed(); // Calculate elapsed time
    println!("{} {}", "[*] Time taken:".blue(),format!("{:?}", duration).green());

    Ok(())
}
