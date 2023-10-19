"""Stubs for hyperdrive math."""
from __future__ import annotations

from . import types

# pylint: disable=unused-argument
# pylint: disable=too-many-arguments

class HyperdriveState:
    """A class representing the hyperdrive contract state."""

    def __new__(cls, pool_config: types.PoolConfig, pool_info: types.PoolInfo) -> HyperdriveState:
        """Create the HyperdriveState instance."""
    def __init__(self, pool_config: types.PoolConfig, pool_info: types.PoolInfo) -> None:
        """Initializes the hyperdrive state.

        Arguments
        ---------
        pool_config : PoolConfig
            Static configuration for the hyperdrive contract.  Set at deploy time.
        pool_info : PoolInfo
            Current state information of the hyperdrive contract.  Includes things like reserve levels and share prices.
        """
    def get_spot_rate(self) -> str:
        """Get the spot rate (fixed rate) for the market.

        Returns
        -------
        str (FixedPoint)
            The spot rate as a string representation of a Solidity uint256 value.
        """
    def get_spot_price(self) -> str:
        """Get the spot price of the bond.

        Returns
        -------
        str (FixedPoint)
            The spot price as a string representation of a Solidity uint256 value.
        """
    def get_out_for_in(self, amount_in: str, shares_in: bool) -> str:
        """Gets the amount of an asset for a given amount in of the other.

        Arguments
        ---------
        amount_in : str (FixedPoint)
            The aount in as a string representation of a Solidity uint256 value.
        shares_in : bool
            True if the asset in is shares, False if it is bonds.

        Returns
        -------
        str (FixedPoint)
            The aount out as a string representation of a Solidity uint256 value.
        """
    def get_out_for_in_safe(self, amount_in: str, shares_in: bool) -> str:
        """Gets the amount of an asset for a given amount in of the other.  Will not cause a panic
        if rust breaks, will return a python error instead.

        Arguments
        ---------
        amount_in : str (FixedPoint)
            The aount in as a string representation of a Solidity uint256 value.
        shares_in : bool
            True if the asset in is shares, False if it is bonds.

        Returns
        -------
        str (FixedPoint)
            The aount out as a string representation of a Solidity uint256 value.
        """
    def get_in_for_out(self, amount_out: str, shares_out: bool) -> str:
        """Gets the amount of an asset for a given amount out of the other.

        Arguments
        ---------
        amount_out : str (FixedPoint)
            The aount out as a string representation of a Solidity uint256 value.
        shares_out : bool
            True if the asset out is shares, False if it is bonds.

        Returns
        -------
        str (FixedPoint)
            The aount in as a string representation of a Solidity uint256 value.
        """
    def to_checkpoint(self, time: str) -> str:
        """Converts a timestamp to the checkpoint timestamp that it corresponds to.

        Arguments
        ---------
        time : str (FixedPoint)
            A string representation of any timestamp before the present.

        Returns
        -------
        str (FixedPoint)
            The checkpoint timestamp as a string representation of a Solidity uint256 value.
        """
    def get_max_long(self, budget: str, checkpoint_exposure: str, maybe_max_iterations: int | None) -> str:
        """Get the max amount of base that can be used to purchase bonds for the given budget.

        Arguments
        ---------
        budget : str (FixedPoint)
            The account budget in base for making a long.
        checkpoint_exposure : str (FixedPoint)
            The net exposure for the given checkpoint.
        maybe_max_iterations : int, optional
            The number of iterations to use for the Newtonian method.

        Returns
        -------
        str (FixedPoint)
            The maximum long, in base, as a string representation of a Solidity uint256 value.
        """
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
            The maximum short as a string representation of a Solidity uint256 value.
        """

def get_max_long(
    pool_config: types.PoolConfig,
    pool_info: types.PoolInfo,
    budget: str,
    checkpoint_exposure: str,
    maybe_max_iterations: int | None,
) -> str:
    """Get the max amount of bonds that can be purchased for the given budget.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.  Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract. Includes things like reserve levels and share prices.
    budget : str (FixedPoint)
        The account budget in base for making a long.
    checkpoint_exposure : str (FixedPoint)
        The net exposure for the given checkpoint.
    maybe_max_iterations : int, optional
        The number of iterations to use for the Newtonian method.

    Returns
    -------
    str (FixedPoint)
        The maximum long as a string representation of a Solidity uint256 value.
    """

def get_max_short(
    pool_config: types.PoolConfig,
    pool_info: types.PoolInfo,
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
        Static configuration for the hyperdrive contract.  Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.  Includes things like reserve levels and share prices.
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
        The maximum short as a string representation of a Solidity uint256 value.
    """

def get_spot_price(
    pool_config: types.PoolConfig,
    pool_info: types.PoolInfo,
) -> str:
    """Get the spot price of the bond.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.  Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.  Includes things like reserve levels and share prices.

    Returns
    -------
    str (FixedPoint)
        The spot price as a string representation of a Solidity uint256 value.
    """

def get_out_for_in(
    pool_config: types.PoolConfig,
    pool_info: types.PoolInfo,
    amount_in: str,
    shares_in: bool,
) -> str:
    """Gets the amount of an asset for a given amount in of the other.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.  Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.  Includes things like reserve levels and share prices.
    amount_in : str (FixedPoint)
        The aount in as a string representation of a Solidity uint256 value.
    shares_in : bool
        True if the asset in is shares, False if it is bonds.

    Returns
    -------
    str (FixedPoint)
        The aount out as a string representation of a Solidity uint256 value.
    """

def get_out_for_in_safe(
    pool_config: types.PoolConfig,
    pool_info: types.PoolInfo,
    amount_in: str,
    shares_in: bool,
) -> str:
    """Gets the amount of an asset for a given amount in of the other.  Will not cause a panic
    if rust breaks, will return a python error instead.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.  Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.  Includes things like reserve levels and share prices.
    amount_in : str (FixedPoint)
        The aount in as a string representation of a Solidity uint256 value.
    shares_in : bool
        True if the asset in is shares, False if it is bonds.

    Returns
    -------
    str (FixedPoint)
        The aount out as a string representation of a Solidity uint256 value.
    """

def get_in_for_out(
    pool_config: types.PoolConfig,
    pool_info: types.PoolInfo,
    amount_out: str,
    shares_out: bool,
) -> str:
    """Gets the amount of an asset for a given amount out of the other.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.  Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.  Includes things like reserve levels and share prices.
    amount_out : str (FixedPoint)
        The aount out as a string representation of a Solidity uint256 value.
    shares_out : bool
        True if the asset out is shares, False if it is bonds.

    Returns
    -------
    str
        The aount in as a string representation of a Solidity uint256 value.
    """

def get_time_stretch(
    rate: str,
) -> str:
    """
    Calculate the time stretch parameter given a pool's spot rate.

    ..math::
        tau = 5.24592 / (0.4665 * r * 100)

    Arguments
    ---------
    rate : str (FixedPoint)
        The pool's spot rate (aka apr, or fixed rate).
    share_adjustment : str (I256)
        The zeta factor for adjusting share reserves.

    Returns
    -------
    time_stretch : str (FixedPoint)
        The time stretch parameter (tau).
    """

def get_effective_share_reserves(
    share_reserves: str,
    share_adjustment: str,
) -> str:
    """
    Calculate the effective share reserves given the share reserves and share adjustment.

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
        The pool's effective share reserves. The
        effective share reserves are a modified version of the share
        reserves used when pricing trades.
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
