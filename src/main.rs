use amms::amm::{bancor::BancorV3Pool, AutomatedMarketMaker, uniswap_v2::UniswapV2Pool};
use ethers::{
    providers::{Http, Provider},
    types::{H160, U256},
};
use std::{str::FromStr, sync::Arc};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_endpoint = std::env::var("ETHEREUM_RPC_ENDPOINT")?;
    let middleware = Arc::new(Provider::<Http>::try_from(rpc_endpoint)?);

    // LINK 
    let token_0 = H160::from_str("0x514910771AF9Ca656af840dff83E8264EcF986CA")?;
    // USDC 
    let token_1 = H160::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")?;

    let contract_address = H160::from_str("0xFD47C74A8030520BACd364FB8e08ACB28766aE7b")?;
    let pool = BancorV3Pool::new_from_address(contract_address, 300, middleware.clone()).await?;

    let link_en = pool.trading_enabled(middleware.clone(), token_0).await?;
    let usdc_en = pool.trading_enabled(middleware.clone(), token_1).await?;
    let output  = pool.test_bancor_comms(middleware.clone(), token_0, token_1).await?;
    let fee     = pool.trade_fee(middleware.clone(), token_0).await?;
    //let output = 0;

    let pool_address = H160::from_str("0xd8c8a2b125527bf97c8e4845b25de7e964468f77")?;
    let pool_v2 = UniswapV2Pool::new_from_address(pool_address, 300, middleware).await?;

    let out = pool_v2.simulate_swap(token_0, U256::from(1000000000000000i64))?;

    let spread = out - output;
    println!("PRICE BACOR: {output}, PRICE UNI: {out}, CURRENT SPREAD: {spread}");

    Ok(())
}
