"""Helper functions for the wrappers."""
from . import types


def _serialize_pool_config(
    pool_config: types.PoolConfigType,
) -> types.PoolConfig:
    return types.PoolConfig(
        baseToken=str(pool_config.baseToken),
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
        oracleSize=str(pool_config.oracleSize),
        updateGap=str(pool_config.updateGap),
    )


def _serialize_pool_info(pool_info: types.PoolInfoType) -> types.PoolInfo:
    return types.PoolInfo(
        shareReserves=str(pool_info.shareReserves),
        shareAdjustment=str(pool_info.shareAdjustment),
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
