"""Types for the hyperdrive contract."""

# contracts have PascalCase names
# pylint: disable=invalid-name
# We do not define the number of instance attributes
# pylint: disable=too-many-instance-attributes
# pylint: disable=too-few-public-methods
from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Protocol


@dataclass
class Fees:
    """Protocal Fees."""

    curve: str
    flat: str
    governanceLP: str
    governanceZombie: str


@dataclass
class PoolConfig:
    """Static configuration for the hyperdrive contract. Set at deploy time."""

    baseToken: str
    linkerFactory: str
    linkerCodeHash: str
    initialVaultSharePrice: str
    minimumShareReserves: str
    minimumTransactionAmount: str
    positionDuration: str
    checkpointDuration: str
    timeStretch: str
    governance: str
    feeCollector: str
    fees: Fees


@dataclass
class PoolInfo:
    """Current state information of the hyperdrive contract. Includes things like reserve levels and share prices."""

    shareReserves: str
    shareAdjustment: str
    zombieBaseProceeds: str
    zombieShareReserves: str
    bondReserves: str
    lpTotalSupply: str
    vaultSharePrice: str
    longsOutstanding: str
    longAverageMaturityTime: str
    shortsOutstanding: str
    shortAverageMaturityTime: str
    withdrawalSharesReadyToWithdraw: str
    withdrawalSharesProceeds: str
    lpSharePrice: str
    longExposure: str


# TODO: pypechain should either use TypedDicts or generate these interfaces.
class CheckpointType(Protocol):
    """Checkpoint struct."""

    vaultSharePrice: int
    longExposure: int


class MarketStateType(Protocol):
    """MarketState struct."""

    shareReserves: int
    bondReserves: int
    shareAdjustment: int
    longExposure: int
    longsOutstanding: int
    shortsOutstanding: int
    longAverageMaturityTime: int
    shortAverageMaturityTime: int
    isInitialized: bool
    isPaused: bool


class FeesType(Protocol):
    """Fees struct."""

    curve: int
    flat: int
    governanceLP: int
    governanceZombie: int


class PoolConfigType(Protocol):
    """PoolConfig struct."""

    baseToken: str
    linkerFactory: str
    linkerCodeHash: bytes
    initialVaultSharePrice: int
    minimumShareReserves: int
    minimumTransactionAmount: int
    positionDuration: int
    checkpointDuration: int
    timeStretch: int
    governance: str
    feeCollector: str
    # TODO: nested Protocol types do not play well with dataclasses.  use 'or Any' for now.
    fees: FeesType | Any


class PoolInfoType(Protocol):
    """PoolInfo struct."""

    shareReserves: int
    shareAdjustment: int
    zombieBaseProceeds: int
    zombieShareReserves: int
    lpTotalSupply: int
    vaultSharePrice: int
    longsOutstanding: int
    longAverageMaturityTime: int
    shortsOutstanding: int
    bondReserves: int
    shortAverageMaturityTime: int
    withdrawalSharesReadyToWithdraw: int
    withdrawalSharesProceeds: int
    lpSharePrice: int
    longExposure: int
