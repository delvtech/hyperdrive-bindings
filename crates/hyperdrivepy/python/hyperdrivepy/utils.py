"""Helper functions for the wrappers."""
# pylint: disable=no-name-in-module
from . import hyperdrivepy as rust_module  # type: ignore
from . import types


def _get_interface(pool_config: types.PoolConfigType, pool_info: types.PoolInfoType) -> rust_module.HyperdriveState:
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface


def _serialize_pool_config(
    pool_config: types.PoolConfigType,
) -> types.PoolConfig:
    return types.PoolConfig(
        baseToken=str(pool_config.baseToken),
        linkerFactory=str(pool_config.linkerFactory),
        linkerCodeHash=pool_config.linkerCodeHash.hex(),  # bytes to string in hex format
        initialSharePrice=str(pool_config.initialSharePrice),
        minimumShareReserves=str(pool_config.minimumShareReserves),
        minimumTransactionAmount=str(pool_config.minimumTransactionAmount),
        positionDuration=str(pool_config.positionDuration),
        checkpointDuration=str(pool_config.checkpointDuration),
        timeStretch=str(pool_config.timeStretch),
        governance=str(pool_config.governance),
        feeCollector=str(pool_config.feeCollector),
        fees=types.Fees(
            curve=str(pool_config.fees.curve),
            flat=str(pool_config.fees.flat),
            governance=str(pool_config.fees.governance),
        ),
    )


def _serialize_pool_info(pool_info: types.PoolInfoType) -> types.PoolInfo:
    return types.PoolInfo(
        shareReserves=str(pool_info.shareReserves),
        shareAdjustment=str(pool_info.shareAdjustment),
        zombieShareReserves=str(pool_info.zombieShareReserves),
        bondReserves=str(pool_info.bondReserves),
        lpTotalSupply=str(pool_info.lpTotalSupply),
        sharePrice=str(pool_info.sharePrice),
        longsOutstanding=str(pool_info.longsOutstanding),
        longAverageMaturityTime=str(pool_info.longAverageMaturityTime),
        shortsOutstanding=str(pool_info.shortsOutstanding),
        shortAverageMaturityTime=str(pool_info.shortAverageMaturityTime),
        withdrawalSharesReadyToWithdraw=str(pool_info.withdrawalSharesReadyToWithdraw),
        withdrawalSharesProceeds=str(pool_info.withdrawalSharesProceeds),
        lpSharePrice=str(pool_info.lpSharePrice),
        longExposure=str(pool_info.longExposure),
    )
