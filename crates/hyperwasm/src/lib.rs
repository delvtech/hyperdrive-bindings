#![allow(non_snake_case)]

mod utils;

use ethers::core::types::{Address, I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_math::State;
use hyperdrive_wrappers::wrappers::i_hyperdrive::{Fees, PoolConfig, PoolInfo};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn getMaxShort(
    state: JsValue,
    budget: String,
    openSharePrice: String,
    checkpointExposure: String,
    maybeConservativePrice: Option<String>,
    maybeMaxIterations: Option<u8>,
) -> String {
    utils::set_panic_hook();
    let _state = State::from(&serde_wasm_bindgen::from_value(state).unwrap());
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

#[wasm_bindgen]
pub fn getMaxLong(
    state: JsValue,
    budget: String,
    checkpointExposure: String,
    maybeMaxIterations: Option<u8>,
) -> String {
    utils::set_panic_hook();
    let _state = State::from(&serde_wasm_bindgen::from_value(state).unwrap());
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

#[wasm_bindgen]
pub fn getSpotPrice(state: JsValue) -> String {
    utils::set_panic_hook();
    let _state = State::from(&serde_wasm_bindgen::from_value(state).unwrap());
    _state.get_spot_price().to_string()
}

#[wasm_bindgen]
pub fn getSpotRate(state: JsValue) -> String {
    utils::set_panic_hook();
    let _state = State::from(&serde_wasm_bindgen::from_value(state).unwrap());
    _state.get_spot_rate().to_string()
}

#[wasm_bindgen]
pub fn getLongAmount(state: JsValue, baseAmount: String) -> String {
    utils::set_panic_hook();
    let _state = State::from(&serde_wasm_bindgen::from_value(state).unwrap());

    _state
        .get_long_amount(U256::from_dec_str(&baseAmount).unwrap())
        .to_string()
}

#[wasm_bindgen]
pub fn getShortDeposit(
    state: JsValue,
    shortAmount: String,
    spotPrice: String,
    openSharePrice: String,
) -> String {
    utils::set_panic_hook();
    let _state = State::from(&serde_wasm_bindgen::from_value(state).unwrap());

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
    pub oracleSize: String,
    pub updateGap: String,
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
            config: PoolConfig {
                base_token: Address::from_str(&wasm_state.config.baseToken).unwrap(),
                governance: Address::from_str(&wasm_state.config.governance).unwrap(),
                fee_collector: Address::from_str(&wasm_state.config.feeCollector).unwrap(),
                fees: Fees {
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
                oracle_size: U256::from_dec_str(&wasm_state.config.oracleSize).unwrap(),
                update_gap: U256::from_dec_str(&wasm_state.config.updateGap).unwrap(),
            },
            info: PoolInfo {
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
