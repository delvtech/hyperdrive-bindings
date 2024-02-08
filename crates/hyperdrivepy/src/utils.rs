use ethers::core::types::{Address, H256, I256, U256};
use hyperdrive_wrappers::wrappers::ihyperdrive::Fees;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

// Helper function to extract U256 values from Python object attributes
pub fn extract_u256_from_attr(ob: &PyAny, attr: &str) -> PyResult<U256> {
    let value_str: String = ob.getattr(attr)?.extract()?;
    U256::from_dec_str(&value_str)
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("Invalid U256 for {}: {}", attr, e)))
}

// Helper function to extract I256 values from Python object attributes
pub fn extract_i256_from_attr(ob: &PyAny, attr: &str) -> PyResult<I256> {
    let value_str: String = ob.getattr(attr)?.extract()?;
    I256::from_dec_str(&value_str)
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("Invalid I256 for {}: {}", attr, e)))
}

// Helper function to extract Ethereum Address values from Python object attributes
pub fn extract_address_from_attr(ob: &PyAny, attr: &str) -> PyResult<Address> {
    let address_str: String = ob.getattr(attr)?.extract()?;
    address_str.parse::<Address>().map_err(|e| {
        PyErr::new::<PyValueError, _>(format!("Invalid Ethereum address for {}: {}", attr, e))
    })
}

// Helper function to extract bytes32 values from Python object attributes
pub fn extract_bytes32_from_attr(ob: &PyAny, attr: &str) -> PyResult<[u8; 32]> {
    let bytes32_str: String = ob.getattr(attr)?.extract()?;
    let bytes32_h256: H256 = bytes32_str.parse::<H256>().map_err(|e| {
        PyErr::new::<PyValueError, _>(format!("Invalid bytes32 for {}: {}", attr, e))
    })?;
    Ok(bytes32_h256.into())
}

pub fn extract_fees_from_attr(ob: &PyAny, attr: &str) -> PyResult<Fees> {
    let fees_obj = ob.getattr(attr)?;

    let curve = extract_u256_from_attr(&fees_obj, "curve")?;
    let flat = extract_u256_from_attr(&fees_obj, "flat")?;
    let governance_lp = extract_u256_from_attr(&fees_obj, "governanceLP")?;
    let governance_zombie = extract_u256_from_attr(&fees_obj, "governanceZombie")?;

    Ok(Fees {
        curve,
        flat,
        governance_lp,
        governance_zombie,
    })
}
