use num_bigint::BigUint;

pub fn format_currency(value: BigUint) -> String {
    let value = value / 100u32;
    format!("$ {value:>2}")
}
