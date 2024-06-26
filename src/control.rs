use crate::word::*;

/// Conditionally move a value from one register to another
/// if the value in the third register is not zero.
/// 
/// # Arguments
/// 
/// * `r` - a mutable reference to the array of registers
/// * `iw` - the current instruction word
/// 
/// # Returns
/// 
/// 0 on success, 1-9 on failure
#[inline]
pub fn cmov(r: &mut [u32; 8], iw: u32) -> u32 {

    // get registers
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;

    // if R[C] != 0, R[A] := R[B]
    if r[c] != 0 {
        r[a] = r[b];
    }

    return 0;
}

/// Halts the program 
/// 
/// # Arguments
/// 
/// * None
/// 
/// # Returns
/// 
/// 70 to signal program halt, 71-79 on failure (which should be impossible)
#[inline]
pub fn halt() -> u32 {
    return 0;
}