pub fn get_nth_bit_from_decimal(decimal: i32, n: usize, digits: usize) -> i32{
    assert!(n < digits, "n must be < digits");
    let shift = digits - 1 - n;
    ((decimal >> shift) & 1) as i32
}