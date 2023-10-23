use std::sync::Arc;

use async_trait::async_trait;
use ethers::{
    abi::{ethabi::Bytes, RawLog, Token},
    prelude::EthEvent,
    providers::Middleware,
    types::{Log, H160, H256, U256, Address},
};
use num_bigfloat::BigFloat;
use serde::{Deserialize, Serialize};

use crate::{
    amm::AutomatedMarketMaker,
    errors::{AMMError, ArithmeticError, EventLogError, SwapSimulationError},
};

use std::{str::FromStr};

use ethers::prelude::abigen;
abigen!(
    IBancorPool, "/home/filip/Dokument/MEV/code/amms-rs/src/amm/bancor/abi.json"
);

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct BancorV3Pool {
    pub address: H160,
    pub token_a: H160,
    pub token_a_decimals: u8,
    pub token_b: H160,
    pub token_b_decimals: u8,
    pub reserve_0: u128,
    pub reserve_1: u128,
    pub fee: u32,
}


impl BancorV3Pool {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        address: H160,
        token_a: H160,
        token_a_decimals: u8,
        token_b: H160,
        token_b_decimals: u8,
        reserve_0: u128,
        reserve_1: u128,
        fee: u32,
    ) -> BancorV3Pool {
        BancorV3Pool {
            address,
            token_a,
            token_a_decimals,
            token_b,
            token_b_decimals,
            reserve_0,
            reserve_1,
            fee,
        }
    }

    pub async fn new_from_address<M: Middleware>(
        pair_address: H160,
        fee: u32,
        middleware: Arc<M>,
    ) -> Result<Self, AMMError<M>> {

        let mut pool = BancorV3Pool {
            address: pair_address,
            token_a: H160::zero(),
            token_a_decimals: 0,
            token_b: H160::zero(),
            token_b_decimals: 0,
            reserve_0: 0,
            reserve_1: 0,
            fee,
        };

        Ok(pool)
    }

    pub fn simulate_swap(&self, token_in: H160, amount_in: U256) -> Result<U256, SwapSimulationError> {
        Ok(U256::from(0))
    } 

    pub fn simulate_swap_mut(
        &mut self,
        token_in: H160,
        amount_in: U256,
    ) -> Result<U256, SwapSimulationError> {
        Ok(U256::from(0))
    }

    pub fn fee(&self) -> u32  {
        self.fee
    }

    pub async fn trade_fee<M: Middleware>(&self, middleware: Arc<M>,token_0: Address) -> Result<U256, AMMError<M>> {
        let network_data = IBancorPool::new(self.address, middleware); 
        let v = match network_data.trading_fee_ppm(token_0).call().await {
            Ok(result) => result,
            Err(contract_error) => return Err(AMMError::ContractError(contract_error)),
        };
        Ok(v.into())
    }

    pub async fn test_bancor_comms<M: Middleware>(&self, middleware: Arc<M>,token_0: Address, token_1: Address) -> Result<U256, AMMError<M>> {
        let network_data = IBancorPool::new(self.address, middleware); 
        let val = U256::from(1000000000000000i64);
        let v = match network_data.trade_output_by_source_amount(token_0, token_1, val).call().await {
            Ok(result) => result,
            Err(contract_error) => return Err(AMMError::ContractError(contract_error)),
        };
        Ok(v.into())
    }

    pub async fn trading_enabled<M: Middleware>(&self, middleware: Arc<M>,token: Address) -> Result<f64, AMMError<M>> {
        let network_data = IBancorPool::new(self.address, middleware); 
        let v = match network_data.trading_enabled(token).call().await {
            Ok(result) => result,
            Err(contract_error) => return Err(AMMError::ContractError(contract_error)),
        };
        Ok(v.into())
    }

    pub fn calculate_price(&self, base_token: H160) -> Result<f64, ArithmeticError> {
        Ok(0.0)
    }

    pub async fn populate_data<M: Middleware>(
        &mut self,
        _block_number: Option<u64>,
        middleware: Arc<M>,
    ) -> Result<(), AMMError<M>> {

        Ok(())
    }

    pub fn tokens(&self) -> Vec<H160> {
        vec![self.token_a, self.token_b]
    }

    pub fn swap_calldata(
        &self,
        amount_0_out: U256,
        amount_1_out: U256,
        to: H160,
        calldata: Vec<u8>,
    ) -> Result<Bytes, ethers::abi::Error> {
        Ok(vec![0])
    }

    pub fn get_token_out(&self, token_in: H160) -> H160 {
        if self.token_a == token_in {
            self.token_b
        } else {
            self.token_a
        }
    }

    pub fn sync_from_log(&mut self, log: Log) -> Result<(), EventLogError> {
        Ok(())
    }

    pub fn sync_on_event_signatures(&self) -> Vec<H256> {
        vec![]
    }

    pub async fn sync<M: Middleware>(&mut self, middleware: Arc<M>) -> Result<(), AMMError<M>> {
        Ok(())
    }
}