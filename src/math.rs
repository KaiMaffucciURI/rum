use::bitpack::*;


/* 
    Add
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 31-39 on failure
*/
#[inline]
pub fn add(r: &mut [u32; 8], iw: u32) -> u32 {
    return 0;
}


/*
    Mul
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 41-49 on failure
*/
#[inline]
pub fn mul(r: &mut [u32; 8], iw: u32) -> u32 {
    return 0;
}

/*
    Div
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 51-59 on failure
*/
#[inline]
pub fn div(r: &mut [u32; 8], iw: u32) -> u32 {
    return 0;
}

/*
    Nand
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 61-69 on failure
*/
#[inline]
pub fn nand(r: &mut [u32; 8], iw: u32) -> u32 {
    return 0;
}