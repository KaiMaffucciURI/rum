use crate::word::*;
use::bitpack::bitpack::*;

/*
    Map: map/allocate new memory segment
    Arguments: pointer to registers, pointer to memory, pointer to saved segment identifiers, instruction word
    Returns: 0 on success, 81-89 on failure
*/
#[inline]
pub fn map(r: &mut [u32; 8], m: &mut Vec<Vec<u32>>, saved_ids: &mut Vec<u32>, iw: u32) -> u32 {

    // get registers
    let args = regs_array(iw);
    let b = args[1] as usize;
    let c = args[2] as usize;

    // if saved_ids isn't empty, pop the last element and store it in r[b]
    if saved_ids.len() > 0 {

        //r[b] = saved_ids.pop().unwrap(); // BAD
        let new_id = saved_ids.pop().unwrap();
        m[new_id as usize] = vec![0; r[c] as usize];
        r[b] = new_id;
        
    } else {

        let new_id = m.len() as u32;
        // push a new segment onto m of length r[c] that is all zeroes
        m.push(vec![0; r[c] as usize]);
        // store the new id in r[b]
        r[b] = new_id;
    }

    return 0;
}

/* 
    Unmap: unmap/deallocate/free memory segment
    Arguments: pointer to registers, pointer to memory, pointer to saved segment identifiers, instruction word
    Returns: 0 on success, 91-99 on failure
*/
#[inline]
pub fn unmap(r: &mut [u32; 8], m: &mut Vec<Vec<u32>>, saved_ids: &mut Vec<u32>, iw: u32) -> u32 {

    // get registers
    let args = regs_array(iw);
    let c = args[2] as usize;

    // if the segment is the last segment in memory
    if r[c] == m.len() as u32 - 1 {

        // pop the last segment off memory
        m.pop();
        
    } else {

        // otherwise, replace the segment with an empty segment
        m[r[c] as usize] = Vec::new();
        // append the identifier to saved_segment_identifiers
        saved_ids.push(r[c] as u32);
    }

    return 0;
}

/*
    Sload: segment load
    Arguments: pointer to registers, pointer to memory, instruction word
    Returns: 0 on success, 11-19 on failure
*/
#[inline]
pub fn sload(r: &mut [u32; 8], m: &mut Vec<Vec<u32>>, iw: u32) -> u32 {
    
    // R[A] := M[R[B]][R[C]]
    // get registers
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;

    // if the segment identifier is out of bounds
    if r[b] as usize >= m.len() {
        return 11;
    }

    // if the offset is out of bounds
    if r[c] as usize >= m[r[b] as usize].len() {
        // print memory segment m[r[b]] as test code
        return 12;
    }

    // load the value at the offset in the segment into the register
    r[a] = m[r[b] as usize][r[c] as usize];

    return 0;
}

/*
    Sstore: segment store
    Arguments: pointer to registers, pointer to memory, instruction word
    Returns: 0 on success, 21-29 on failure
*/
#[inline]
pub fn sstore(r: &mut [u32; 8], m: &mut Vec<Vec<u32>>, iw: u32) -> u32 {
    
    // M[R[A]][R[B]] := R[C]

    // get registers
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;

    // if the segment identifier is out of bounds
    if r[a] as usize >= m.len() {
        return 21;
    }

    // if the offset is out of bounds, also return an error
    if r[b] as usize >= m[r[a] as usize].len() {
        //   reduce the offset until we reach a segment either at the end or a valid index?
        return 22;
    }

    m[r[a] as usize][r[b] as usize] = r[c];
    
    return 0;
}

/*
    Loadp: load program
    Arguments: pointer to registers, pointer to memory, instruction word, pointer to program counter
    Returns: 0 on success, 121-129 on failure
*/
#[inline]
pub fn loadp(r: &mut [u32; 8], m: &mut Vec<Vec<u32>>, iw: u32, pc: &mut i64) -> u32 {

    // get registers
    let args = regs_array(iw);
    let b = args[1] as usize;
    let c = args[2] as usize;

    // if the segment identifier is out of bounds
    if r[b] as usize >= m.len() { return 121 }

    // if the offset is out of bounds
    if r[c] as usize >= m[r[b] as usize].len() { return 122 }

    // if the segment is not the first segment,
    if r[b] != 0 {

        // duplicate the segment
        m[0] = m[r[b] as usize].clone();
    }

    // set the program counter to the offset in the segment
    *pc = r[c] as i64;

    return 0;
}

/*
    Loadv: load value
    Arguments: pointer to registers, pointer to memory, instruction word
    Returns: 0 on success, 131-139 on failure
*/
#[inline]
pub fn loadv(r: &mut [u32; 8], iw: u32) -> u32 {

    // get register a from the 3 bits immediately less significant than the opcode
    let a = getu(iw as u64, 3, 25).unwrap() as usize;

    // use getu to load the value into the register
    r[a] = getu(iw as u64, 25, 0).unwrap() as u32;

    return 0;
}