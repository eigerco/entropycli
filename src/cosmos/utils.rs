use cosmrs::Gas;

pub fn mul_gas_float(gas: impl Into<Gas>, f: f64) -> Gas {
    #[allow(clippy::cast_precision_loss)]
    let gas = gas.into() as f64;

    let gas = (gas * f).ceil();


    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    let gas = gas as u64;
    Gas::from(gas)
}
