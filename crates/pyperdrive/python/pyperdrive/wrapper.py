"""Python wrapper for the rust module."""
from __future__ import annotations

# The module itself doesn't have type hints, so we will ignore warnings in this file.
# pylint: disable=c-extension-no-member
# pylint: disable=no-name-in-module
from . import pyperdrive as rust_module  # type: ignore
from . import types

# we don't control the number of arguments, wrapping rust functions
# pylint: disable=too-many-arguments


class HyperdriveState:
    """A python class representing the hyperdrive contract state."""

    _rust_interface: rust_module.HyperdriveState

    def __init__(self, pool_config: types.PoolConfigType, pool_info: types.PoolInfoType) -> None:
        """Initializes the hyperdrive state.

        Arguments
        ---------
        pool_config : PoolConfig
            Static configuration for the hyperdrive contract. Set at deploy time.
        pool_info : PoolInfo
            Current state information of the hyperdrive contract.
            Includes things like reserve levels and share prices.
        """
        pool_config_serialized = _serialize_pool_config_values(pool_config)
        pool_info_serialized = _serialize_pool_info_values(pool_info)
        self._rust_interface = rust_module.HyperdriveState(pool_config_serialized, pool_info_serialized)

    def get_spot_rate(self) -> str:
        """Get the spot rate (fixed rate) for the market.

        Returns
        -------
        str (FixedPoint)
            The pool's spot rate.
        """
        return self._rust_interface.get_spot_rate()

    def get_spot_price(self) -> str:
        """Get the spot price of the bond.

        Returns
        -------
        str (FixedPoint)
            The pool's spot price.
        """
        return self._rust_interface.get_spot_price()

    def get_out_for_in(self, amount_in: str, shares_in: bool):
        """Gets the amount of an asset for a given amount in of the other.

        Arguments
        ---------
        amount_in : str (FixedPoint)
            The amount of asset going into the pool.
        shares_in : bool
            True if the asset in is shares, False if it is bonds.
            The amount out will be the opposite type.

        Returns
        -------
        str (FixedPoint)
            The amount out.
        """
        return self._rust_interface.get_out_for_in(amount_in, shares_in)

    def get_out_for_in_safe(self, amount_in: str, shares_in: bool):
        """Gets the amount of an asset for a given amount in of the other.
        Will not cause a panic if rust breaks, will return a python error instead.

        Arguments
        ---------
        amount_in : str (FixedPoint)
            The amount of asset going into the pool.
        shares_in : bool
            True if the asset in is shares, False if it is bonds.
            The amount out will be the opposite type.

        Returns
        -------
        str (FixedPoint)
            The amount out.
        """
        return self._rust_interface.get_out_for_in_safe(amount_in, shares_in)

    def get_in_for_out(self, amount_out: str, shares_out: bool):
        """Gets the amount of an asset for a given amount out of the other.

        Arguments
        ---------
        amount_out : str (FixedPoint)
            The amount of asset the user expects to receive from the pool.
        shares_out : bool
            True if the asset out is shares, False if it is bonds.
            The amount in will be the opposite type.

        Returns
        -------
        str (FixedPoint)
            The amount in as a string representation of a Solidity uint256 value.
        """
        return self._rust_interface.get_out_for_in_safe(amount_out, shares_out)

    def to_checkpoint(self, time: str) -> str:
        """Converts a timestamp to the checkpoint timestamp that it corresponds to.

        Arguments
        ---------
        time : str (U256)
            A string representation of any timestamp (in seconds) before or at the present.

        Returns
        -------
        str (U256)
            The checkpoint timestamp.
        """

        return self._rust_interface.to_checkpoint(time)

    def get_max_long(self, budget: str, checkpoint_exposure: str, maybe_max_iterations: int | None) -> str:
        """Get the max amount of bonds that can be purchased for the given budget.

        Arguments
        ---------
        budget : str (FixedPont)
            The account budget in base for making a long.
        checkpoint_exposure : str (I256)
            The net exposure for the given checkpoint.
        maybe_max_iterations : int, optional
            The number of iterations to use for the Newtonian method.

        Returns
        -------
        str (FixedPoint)
            The maximum long the pool and user's wallet can support.
        """
        return self._rust_interface.get_max_long(budget, checkpoint_exposure, maybe_max_iterations)

    def get_max_short(
        self,
        budget: str,
        open_share_price: str,
        checkpoint_exposure: str,
        maybe_conservative_price: str | None,
        maybe_max_iterations: int | None,
    ) -> str:
        """Get the max amount of bonds that can be shorted for the given budget.

        Arguments
        ---------
        budget : str (FixedPoint)
            The account budget in base for making a short.
        open_share_price : str (FixedPoint)
            The share price of underlying vault.
        checkpoint_exposure : str (FixedPoint)
            The net exposure for the given checkpoint.
        maybe_conservative_price : str (FixedPoint), optional
            A lower bound on the realized price that the short will pay.
        maybe_max_iterations : int, optional
            The number of iterations to use for the Newtonian method.

        Returns
        -------
        str (FixedPoint)
            The maximum short the pool and user's wallet can handle.
        """
        return self._rust_interface.get_max_short(
            budget,
            open_share_price,
            checkpoint_exposure,
            maybe_conservative_price,
            maybe_max_iterations,
        )


def get_max_long(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    budget: str,
    checkpoint_exposure: str,
    maybe_max_iterations: int | None,
) -> str:
    """Get the max amount of bonds that can be purchased for the given budget.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    budget : str (FixedPont)
        The account budget in base for making a long.
    checkpoint_exposure : str (I256)
        The net exposure for the given checkpoint.
    maybe_max_iterations : int, optional
        The number of iterations to use for the Newtonian method.

    Returns
    -------
    str (FixedPoint)
        The maximum long the pool and user's wallet can support.
    """
    return rust_module.get_max_long(
        _serialize_pool_config_values(pool_config),
        _serialize_pool_info_values(pool_info),
        budget,
        checkpoint_exposure,
        maybe_max_iterations,
    )


def get_max_short(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    budget: str,
    open_share_price: str,
    checkpoint_exposure: str,
    maybe_conservative_price: str | None,
    maybe_max_iterations: int | None,
) -> str:
    """Get the max amount of bonds that can be shorted for the given budget.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    budget : str (FixedPoint)
        The account budget in base for making a short.
    open_share_price : str (FixedPoint)
        The share price of underlying vault.
    checkpoint_exposure : str (FixedPoint)
        The net exposure for the given checkpoint.
    maybe_conservative_price : str (FixedPoint), optional
        A lower bound on the realized price that the short will pay.
    maybe_max_iterations : int, optional
        The number of iterations to use for the Newtonian method.

    Returns
    -------
    str (FixedPoint)
        The maximum short the pool and user's wallet can handle.
    """
    return rust_module.get_max_short(
        _serialize_pool_config_values(pool_config),
        _serialize_pool_info_values(pool_info),
        budget,
        open_share_price,
        checkpoint_exposure,
        maybe_conservative_price,
        maybe_max_iterations,
    )


def get_spot_price(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
) -> str:
    """Get the spot price of the bond.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.

    Returns
    -------
    str (FixedPoint)
        The pool's spot price.
    """
    return rust_module.get_spot_price(
        _serialize_pool_config_values(pool_config),
        _serialize_pool_info_values(pool_info),
    )


def get_spot_rate(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
) -> str:
    """Get the spot rate (fixed rate) for the market.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.


    Returns
    -------
    str (FixedPoint)
        The pool's spot rate.
    """
    return rust_module.get_spot_rate(
        _serialize_pool_config_values(pool_config),
        _serialize_pool_info_values(pool_info),
    )


def get_out_for_in(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    amount_in: str,
    shares_in: bool,
) -> str:
    """Gets the amount of an asset for a given amount in of the other.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    amount_in : str (FixedPoint)
        The amount of asset going into the pool.
    shares_in : bool
        True if the asset in is shares, False if it is bonds.
        The amount out will be the opposite type.

    Returns
    -------
    str (FixedPoint)
        The amount out.
    """
    return rust_module.get_out_for_in(
        _serialize_pool_config_values(pool_config),
        _serialize_pool_info_values(pool_info),
        amount_in,
        shares_in,
    )


def get_out_for_in_safe(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    amount_in: str,
    shares_in: bool,
) -> str:
    """Gets the amount of an asset for a given amount in of the other.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    amount_in : str (FixedPoint)
        The amount of asset going into the pool.
    shares_in : bool
        True if the asset in is shares, False if it is bonds.
        The amount out will be the opposite type.

    Returns
    -------
    str (FixedPoint)
        The amount out.
    """
    return rust_module.get_out_for_in_safe(
        _serialize_pool_config_values(pool_config),
        _serialize_pool_info_values(pool_info),
        amount_in,
        shares_in,
    )


def get_in_for_out(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    amount_out: str,
    shares_out: bool,
) -> str:
    """Gets the amount of an asset for a given amount out of the other.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    amount_out : str (FixedPoint)
        The amount of asset the user expects to receive from the pool.
    shares_out : bool
        True if the asset out is shares, False if it is bonds.
        The amount in will be the opposite type.

    Returns
    -------
    str (FixedPoint)
        The amount in as a string representation of a Solidity uint256 value.
    """
    return rust_module.get_in_for_out(
        _serialize_pool_config_values(pool_config),
        _serialize_pool_info_values(pool_info),
        amount_out,
        shares_out,
    )


def get_time_stretch(
    rate: str,
) -> str:
    """Calculate the time stretch parameter given a pool's spot rate.

    ..math::
        tau = 5.24592 / (0.4665 * r * 100)

    Arguments
    ---------
    rate : str (FixedPoint)
        The pool's spot rate (aka apr, or fixed rate).

    Returns
    -------
    time_stretch : str (FixedPoint)
        The time stretch parameter (tau).
    """
    return rust_module.get_time_stretch(rate)


def get_effective_share_reserves(
    share_reserves: str,
    share_adjustment: str,
) -> str:
    r"""Calculate the effective share reserves given the share reserves and share adjustment.

    ..math::
        z_effective = z - \zeta

    Arguments
    ---------
    share_reserves : str (FixedPoint)
        The pool's share reserves.
    share_adjustment : str (I256)
        The zeta factor for adjusting share reserves.

    Returns
    -------
    effective_share_reserves : str (FixedPoint)
        The adjusted share reserves, accounting for the zeta factor.
    """
    return rust_module.get_effective_share_reserves(share_reserves, share_adjustment)


def calculate_bonds_given_shares_and_rate(
    effective_share_reserves: str,
    initial_share_price: str,
    apr: str,
    position_duration: str,
    time_stretch: str,
) -> str:
    """
    Calculates the bond reserves assuming that the pool has a given
    share reserves and fixed rate APR.

    ..math::
        r = ((1/p)-1)/t = (1-p)/(pt)
        p = ((u * z) / y) ** t
        y = mu * (z - zeta) * (1 + apr * t) ** (1/tau)

    Arguments
    ---------
    effective_share_reserves : str (FixedPoint)
        The pool's effective share reserves. The effective share
        reserves are a modified version of the share reserves
        used when pricing trades.
    initial_share_price : str (FixedPoint)
        The pool's initial share price.
    apr : str (FixedPoint)
        The pool's APR.
    position_duration : str (FixedPoint)
        The amount of time until maturity in seconds.
    time_stretch : str (FixedPoint)
        The time stretch parameter (tau).

    Returns
    -------
    bond_reserves : str (FixedPoint)
        The bond reserves (without adjustment) that make
        the pool have a specified APR.
    """
    return rust_module.calculate_bonds_given_shares_and_rate(
        effective_share_reserves, initial_share_price, apr, position_duration, time_stretch
    )


def _serialize_pool_config_values(
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


def _serialize_pool_info_values(pool_info: types.PoolInfoType) -> types.PoolInfo:
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
