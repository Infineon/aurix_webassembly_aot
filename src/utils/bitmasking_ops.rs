
/// computes effective available space for the linear memory from the allocated one.
/// This computation is used for the implementation of bitmask-based sandboxing.
/// Since the sandboxing scheme requires a space whose size is a power of two,
/// the effective available space is the highest power of two less or equal than the allocated space,
/// after reserving a buffer of 7 Bytes to make memory access operations at the higher boundary safe.
/// 
///  # Arguments
///  
///  * `allocated_memory` - size of the allocated memory in Bytes
/// 
///  # Returns
///  
///  the effective available space in Bytes
///  
/// cf. https://codeforces.com/blog/entry/138850
#[cfg(feature="address-masking")]
pub fn compute_effective_sandboxed_memory_space(allocated_space_size: u32) -> u32 {
    // No memory access can be guaranteed to be safe if the space is less than 8 Bytes.
    if allocated_space_size < 8 {
        return 0;
    }

    // reserve buffer for boundary  memory access
    let mut result = allocated_space_size-7;

    //set all bits that are less significant than the MSB
    result>>=1;

    result |= result>>1;
    result |= result>>2;
    result |= result>>4;
    result |= result>>8;
    result |= result>>16;

    result+1
}
