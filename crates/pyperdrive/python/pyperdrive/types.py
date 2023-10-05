"""Types for the hyperdrive contract."""
# contracts have PascalCase names
# pylint: disable=invalid-name
# We do not define the number of instance attributes
# pylint: disable=too-many-instance-attributes
from __future__ import annotations

from dataclasses import dataclass


@dataclass
class Fees:
    """Protocal Fees."""

    curve: str
    flat: str
    governance: str


@dataclass
class PoolConfig:
    """Static configuration for the hyperdrive contract. Set at deploy time."""

    baseToken: str
    initialSharePrice: str
    minimumShareReserves: str
    minimumTransactionAmount: str
    positionDuration: str
    checkpointDuration: str
    timeStretch: str
    governance: str
    feeCollector: str
    Fees: Fees
    oracleSize: str
    updateGap: str


@dataclass
class PoolInfo:
    """Current state information of the hyperdrive contract. Includes things like reserve levels and share prices."""

    shareReserves: str
    shareAdjustment: str
    bondReserves: str
    lpTotalSupply: str
    sharePrice: str
    longsOutstanding: str
    longAverageMaturityTime: str
    shortsOutstanding: str
    shortAverageMaturityTime: str
    withdrawalSharesReadyToWithdraw: str
    withdrawalSharesProceeds: str
    lpSharePrice: str
    longExposure: str
