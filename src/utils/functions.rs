use num_bigint::BigUint;

pub fn format_currency(value: &BigUint) -> String {
    let integer_part = value / 100u32;
    let fractional_part = value % 100u32;
    format!("$ {integer_part}.{fractional_part:02}")
}
