#![allow(non_snake_case)]

mod utils;

use ethers::core::types::{Address, I256, U256};
use fixed_point::FixedPoint;
use fixed_point_macros::fixed;
use hyperdrive_math::State;
use hyperdrive_wrappers::wrappers::i_hyperdrive::{
    Fees as _Fees, PoolConfig as _PoolConfig, PoolInfo as _PoolInfo,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const FEES: &'static str = r#"
interface Fees {
    curve: string;
    flat: string;
    governance: string;
}"#;

#[wasm_bindgen(typescript_custom_section)]
const POOL_CONFIG: &'static str = r#"
interface PoolConfig {
    baseToken: string,
    initialSharePrice: string,
    minimumShareReserves: string,
    minimumTransactionAmount: string,
    positionDuration: string,
    checkpointDuration: string,
    timeStretch: string,
    governance: string,
    feeCollector: string,
    fees: Fees,
    linkerFactory: string,
    linkerCodeHash: string,
    precisionThreshold: string,
}"#;

#[wasm_bindgen(typescript_custom_section)]
const POOL_INFO: &'static str = r#"
interface PoolInfo {
    shareReserves: string,
    shareAdjustment: string,
    bondReserves: string,
    lpTotalSupply: string,
    sharePrice: string,
    longsOutstanding: string,
    longAverageMaturityTime: string,
    shortsOutstanding: string,
    shortAverageMaturityTime: string,
    withdrawalSharesReadyToWithdraw: string,
    withdrawalSharesProceeds: string,
    lpSharePrice: string,
    longExposure: string,
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Fees")]
    pub type Fees;

    #[wasm_bindgen(typescript_type = "PoolConfig")]
    pub type PoolConfig;

    #[wasm_bindgen(typescript_type = "PoolInfo")]
    pub type PoolInfo;
}

/// Get the max amount of longs that can be shorted given the current state of
/// the pool.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param openSharePrice - The open share price of the pool's current
/// checkpoint
///
/// @param checkpointExposure - The exposure of the pool's current checkpoint
///
/// @param maybeConservativePrice - A lower bound on the realized price that the
/// short will pay. This is used to help the algorithm converge faster in real
/// world situations. If this is `None`, then we'll use the theoretical worst
/// case realized price.
///
/// @param maybeMaxIterations - The maximum number of iterations to run the
/// binary search for
#[wasm_bindgen(skip_jsdoc)]
pub fn getMaxShort(
    poolInfo: &PoolInfo,
    poolConfig: &PoolConfig,
    budget: String,
    openSharePrice: String,
    checkpointExposure: String,
    maybeConservativePrice: Option<String>,
    maybeMaxIterations: Option<u8>,
) -> String {
    utils::set_panic_hook();
    let _state = State::from(&WasmState {
        info: serde_wasm_bindgen::from_value(poolInfo.into()).unwrap(),
        config: serde_wasm_bindgen::from_value(poolConfig.into()).unwrap(),
    });
    let _budget = U256::from_dec_str(&budget).unwrap();
    let checkpoint_exposure: I256 =
        I256::from_raw(U256::from_dec_str(&checkpointExposure).unwrap());
    let open_share_price: I256 = I256::from_raw(U256::from_dec_str(&openSharePrice).unwrap());

    let _maybe_conservative_price: Option<FixedPoint> = maybeConservativePrice
        .as_ref()
        .map(|price_str| FixedPoint::from(U256::from_dec_str(price_str).unwrap()));

    _state
        .get_max_short(
            _budget,
            open_share_price,
            checkpoint_exposure,
            _maybe_conservative_price,
            maybeMaxIterations.map(|x| x.into()),
        )
        .to_string()
}

/// Gets the number of bonds received when opening a long for a given base
/// amount.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param baseAmount - The amount of base tokens to open a long for
#[wasm_bindgen(skip_jsdoc)]
pub fn calcOpenLong(poolInfo: &PoolInfo, poolConfig: &PoolConfig, baseAmount: String) -> String {
    utils::set_panic_hook();
    let _state = State::from(&WasmState {
        info: serde_wasm_bindgen::from_value(poolInfo.into()).unwrap(),
        config: serde_wasm_bindgen::from_value(poolConfig.into()).unwrap(),
    });
    let base_amount: FixedPoint = FixedPoint::from(U256::from_dec_str(&baseAmount).unwrap());

    _state.get_long_amount(base_amount).to_string()
}

/// Gets the amount of base the trader will need to deposit for a short of
/// a given number of bonds.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param shortAmount - The amount of bonds to short
#[wasm_bindgen(skip_jsdoc)]
pub fn calcOpenShort(poolInfo: &PoolInfo, poolConfig: &PoolConfig, shortAmount: String) -> String {
    utils::set_panic_hook();
    let _state = State::from(&WasmState {
        info: serde_wasm_bindgen::from_value(poolInfo.into()).unwrap(),
        config: serde_wasm_bindgen::from_value(poolConfig.into()).unwrap(),
    });
    let short_amount: FixedPoint = FixedPoint::from(U256::from_dec_str(&shortAmount).unwrap());

    let spot_price = _state.get_spot_price();

    _state
        .get_short_deposit(short_amount, spot_price, fixed!(0))
        .unwrap()
        .to_string()
}

/// Get the max amount of base tokens that can be spent on a long position
/// given the current state of the pool.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param budget - The maximum amount of base tokens that can be spent
///
/// @param checkpointExposure - The exposure of the pool's current checkpoint
///
/// @param maybeMaxIterations - The maximum number of iterations to run the
/// binary search for
#[wasm_bindgen(skip_jsdoc)]
pub fn getMaxLong(
    poolInfo: &PoolInfo,
    poolConfig: &PoolConfig,
    budget: String,
    checkpointExposure: String,
    maybeMaxIterations: Option<u8>,
) -> String {
    utils::set_panic_hook();
    let _state = State::from(&WasmState {
        info: serde_wasm_bindgen::from_value(poolInfo.into()).unwrap(),
        config: serde_wasm_bindgen::from_value(poolConfig.into()).unwrap(),
    });
    let _budget = U256::from_dec_str(&budget).unwrap();
    let checkpoint_exposure: I256 = I256::from_dec_str(&checkpointExposure).unwrap();

    _state
        .get_max_long(
            _budget,
            checkpoint_exposure,
            maybeMaxIterations.map(|x| x.into()),
        )
        .to_string()
}

/// Get the price of a single long token given the current state of the pool.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
#[wasm_bindgen(skip_jsdoc)]
pub fn getSpotPrice(poolInfo: &PoolInfo, poolConfig: &PoolConfig) -> String {
    utils::set_panic_hook();
    let _state = State::from(&WasmState {
        info: serde_wasm_bindgen::from_value(poolInfo.into()).unwrap(),
        config: serde_wasm_bindgen::from_value(poolConfig.into()).unwrap(),
    });
    _state.get_spot_price().to_string()
}

/// Gets the pool's fixed APR, i.e. the fixed rate a user locks in when they
/// open a long.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
#[wasm_bindgen(skip_jsdoc)]
pub fn getSpotRate(poolInfo: &PoolInfo, poolConfig: &PoolConfig) -> String {
    utils::set_panic_hook();
    let _state = State::from(&WasmState {
        info: serde_wasm_bindgen::from_value(poolInfo.into()).unwrap(),
        config: serde_wasm_bindgen::from_value(poolConfig.into()).unwrap(),
    });
    _state.get_spot_rate().to_string()
}

/// Gets the long amount that will be opened for a given base amount.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param baseAmount - The amount of base tokens to open a long for
#[wasm_bindgen(skip_jsdoc)]
pub fn getLongAmount(poolInfo: &PoolInfo, poolConfig: &PoolConfig, baseAmount: String) -> String {
    utils::set_panic_hook();
    let _state = State::from(&WasmState {
        info: serde_wasm_bindgen::from_value(poolInfo.into()).unwrap(),
        config: serde_wasm_bindgen::from_value(poolConfig.into()).unwrap(),
    });

    _state
        .get_long_amount(U256::from_dec_str(&baseAmount).unwrap())
        .to_string()
}

/// Gets the amount of base the trader will need to deposit for a short of a
/// given size.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param shortAmount - The amount of longs to short
///
/// @param openSharePrice - The open share price of the pool's current
/// checkpoint
#[wasm_bindgen(skip_jsdoc)]
pub fn getShortDeposit(
    poolInfo: &PoolInfo,
    poolConfig: &PoolConfig,
    shortAmount: String,
    spotPrice: String,
    openSharePrice: String,
) -> String {
    utils::set_panic_hook();
    let _state = State::from(&WasmState {
        info: serde_wasm_bindgen::from_value(poolInfo.into()).unwrap(),
        config: serde_wasm_bindgen::from_value(poolConfig.into()).unwrap(),
    });

    _state
        .get_short_deposit(
            FixedPoint::from(U256::from_dec_str(&shortAmount).unwrap()),
            FixedPoint::from(I256::from_raw(U256::from_dec_str(&spotPrice).unwrap())),
            FixedPoint::from(I256::from_raw(U256::from_dec_str(&openSharePrice).unwrap())),
        )
        .unwrap()
        .to_string()
}

#[derive(Serialize, Deserialize)]
pub struct WasmFees {
    pub curve: String,
    pub flat: String,
    pub governance: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmPoolConfig {
    pub baseToken: String,
    pub initialSharePrice: String,
    pub minimumShareReserves: String,
    pub minimumTransactionAmount: String,
    pub positionDuration: String,
    pub checkpointDuration: String,
    pub timeStretch: String,
    pub governance: String,
    pub feeCollector: String,
    pub fees: WasmFees,
    pub linkerFactory: String,
    pub linkerCodeHash: String,
    pub precisionThreshold: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmPoolInfo {
    pub shareReserves: String,
    pub shareAdjustment: String,
    pub bondReserves: String,
    pub lpTotalSupply: String,
    pub sharePrice: String,
    pub longsOutstanding: String,
    pub longAverageMaturityTime: String,
    pub shortsOutstanding: String,
    pub shortAverageMaturityTime: String,
    pub withdrawalSharesReadyToWithdraw: String,
    pub withdrawalSharesProceeds: String,
    pub lpSharePrice: String,
    pub longExposure: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmState {
    pub config: WasmPoolConfig,
    pub info: WasmPoolInfo,
}

impl From<&WasmState> for State {
    fn from(wasm_state: &WasmState) -> State {
        State {
            config: _PoolConfig {
                base_token: Address::from_str(&wasm_state.config.baseToken).unwrap(),
                governance: Address::from_str(&wasm_state.config.governance).unwrap(),
                fee_collector: Address::from_str(&wasm_state.config.feeCollector).unwrap(),
                fees: _Fees {
                    curve: U256::from_dec_str(&wasm_state.config.fees.curve).unwrap(),
                    flat: U256::from_dec_str(&wasm_state.config.fees.flat).unwrap(),
                    governance: U256::from_dec_str(&wasm_state.config.fees.governance).unwrap(),
                },
                initial_share_price: U256::from_dec_str(&wasm_state.config.initialSharePrice)
                    .unwrap(),
                minimum_share_reserves: U256::from_dec_str(&wasm_state.config.minimumShareReserves)
                    .unwrap(),
                minimum_transaction_amount: U256::from_dec_str(
                    &wasm_state.config.minimumTransactionAmount,
                )
                .unwrap(),
                time_stretch: U256::from_dec_str(&wasm_state.config.timeStretch).unwrap(),
                position_duration: U256::from_dec_str(&wasm_state.config.positionDuration).unwrap(),
                checkpoint_duration: U256::from_dec_str(&wasm_state.config.checkpointDuration)
                    .unwrap(),
                linker_factory: Address::from_str(&wasm_state.config.linkerFactory).unwrap(),
                linker_code_hash: hex::decode(&wasm_state.config.linkerCodeHash)
                    .unwrap()
                    .try_into()
                    .unwrap(),
                precision_threshold: U256::from_dec_str(&wasm_state.config.precisionThreshold)
                    .unwrap(),
            },
            info: _PoolInfo {
                share_reserves: U256::from_dec_str(&wasm_state.info.shareReserves).unwrap(),
                share_adjustment: I256::from_dec_str(&wasm_state.info.shareAdjustment).unwrap(),
                bond_reserves: U256::from_dec_str(&wasm_state.info.bondReserves).unwrap(),
                long_exposure: U256::from_dec_str(&wasm_state.info.longExposure).unwrap(),
                share_price: U256::from_dec_str(&wasm_state.info.sharePrice).unwrap(),
                longs_outstanding: U256::from_dec_str(&wasm_state.info.longsOutstanding).unwrap(),
                shorts_outstanding: U256::from_dec_str(&wasm_state.info.shortsOutstanding).unwrap(),
                long_average_maturity_time: U256::from_dec_str(
                    &wasm_state.info.longAverageMaturityTime,
                )
                .unwrap(),
                short_average_maturity_time: U256::from_dec_str(
                    &wasm_state.info.shortAverageMaturityTime,
                )
                .unwrap(),
                lp_total_supply: U256::from_dec_str(&wasm_state.info.lpTotalSupply).unwrap(),
                lp_share_price: U256::from_dec_str(&wasm_state.info.lpSharePrice).unwrap(),
                withdrawal_shares_proceeds: U256::from_dec_str(
                    &wasm_state.info.withdrawalSharesProceeds,
                )
                .unwrap(),
                withdrawal_shares_ready_to_withdraw: U256::from_dec_str(
                    &wasm_state.info.withdrawalSharesReadyToWithdraw,
                )
                .unwrap(),
            },
        }
    }
}
