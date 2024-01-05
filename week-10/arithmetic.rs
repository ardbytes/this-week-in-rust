fn main() {
    println!(
        "Wrapping multiply   : 150_u8 * 2 = {}",
        150_u8.wrapping_mul(2)
    );

    println!(
        "Saturating multiply : 150_u8 * 2 = {}",
        150_u8.saturating_mul(2)
    );

    println!(
        "Checked multiply    : 150_u8 * 2 = {:?}",
        150_u8.checked_mul(2)
    );

    println!(
        "Overflowing multiply: 150_u8 * 2 = {:?}",
        150_u8.overflowing_mul(2)
    );
}
