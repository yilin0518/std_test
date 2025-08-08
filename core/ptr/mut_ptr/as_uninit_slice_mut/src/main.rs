#![feature(ptr_as_uninit, slice_ptr_get)]
use std::fmt::Debug;

// Unsound function that violates multiple SPs based on different paths
fn case_as_uninit_slice_mut1<'a, T: Default + Clone + Debug + Copy>(
    slice_ptr: *mut [T],
    len: usize,
    path: u32,
    offset: isize,
) -> bool{
    match path {
        0 => {
            // Path 0: Normal case
            unsafe { slice_ptr.as_uninit_slice_mut();
            println!("{:?}", slice_ptr);
            }
            true
        }
        1 => {
            // Path 1: Aligned
            // We need to work with the raw pointer and length separately
            let ptr = slice_ptr.as_mut_ptr();
            let len = slice_ptr.len();
            let misaligned_ptr = unsafe { (ptr as *mut u8).offset(offset) as *mut T };
            let new_slice_ptr = std::ptr::slice_from_raw_parts_mut(misaligned_ptr, len);
            unsafe { 
                new_slice_ptr.as_uninit_slice_mut();
                println!("{:?}", new_slice_ptr);
            }
            true
        }
        2 => {
            // Path 2: ZeroLen + Aligned
            use std::slice;
            #[repr(align(1))]
            #[derive(Copy, Clone, Debug)]
            struct Aligned8();
            let mut var = [Aligned8(); 0];
            //let var_ptr = &var as *const u64;
            let slice_ptr = &mut var as *mut [Aligned8];
            let ptr = slice_ptr.as_mut_ptr();
            let len = slice_ptr.len();
            println!("len: {}", len);
            let misaligned_ptr = unsafe { ptr.byte_offset(offset) };
            let new_slice_ptr = std::ptr::slice_from_raw_parts_mut(misaligned_ptr, len);
            unsafe { 
                println!("new_slice_ptr: {:?}", new_slice_ptr);
                new_slice_ptr.as_uninit_slice_mut();
                println!("{:?}", new_slice_ptr);
            }
            true
        }
        3 => {
            // Path 3: Alias - create multiple aliases to same data
            let mut slice = [T::default(); 5];
            unsafe {
                let ptr = &mut slice as *mut [T];
                let val = ptr.as_uninit_slice_mut();
                slice[0] = T::default();
                println!("{:?}", val);
            }
            true
        }
        4 => {
            // Path 4: ValidNum - create slice where len * size_of::<T>() > isize::MAX
            // We need to choose a type and length such that the total size overflows isize
            
            // For a 1-byte type like u8, we need len > isize::MAX
            // For a 4-byte type like u32, we need len > isize::MAX / 4
            // For an 8-byte type like u64, we need len > isize::MAX / 8
            
            let ptr = slice_ptr.as_mut_ptr();
            let original_len = slice_ptr.len();
            
            // Calculate the maximum safe length for this type
            let max_safe_len = isize::MAX as usize / std::mem::size_of::<T>();
            
            // Create a slice with length that will cause overflow
            let overflow_len = max_safe_len + 1;
            
            // Create the overflow slice
            let overflow_slice_ptr = std::ptr::slice_from_raw_parts_mut(ptr, overflow_len);
            
            unsafe { 
                println!("overflow_len: {}", overflow_len);
                overflow_slice_ptr.as_uninit_slice_mut();
                println!("Created slice with len: {}, size: {} bytes", 
                         overflow_len, 
                         overflow_len * std::mem::size_of::<T>());
                println!("{:?}", overflow_slice_ptr);
            }
            true
        }
        5 => {
            use std::slice;
            // Path 5: Single Allocation - multiple allocations with single pointer
            // Create two separate allocations that might be contiguous
            fn join_slices<'a>(fst: &'a mut [i8], snd: &'a mut [i8]) -> bool {
                let fst_end = fst.as_ptr().wrapping_add(fst.len());
                let snd_start = snd.as_ptr();
                assert_eq!(fst_end, snd_start, "Slices must be contiguous!");
                unsafe {
                    // The assertion above ensures `fst` and `snd` are contiguous, but they might
                    // still be contained within _different allocations_, in which case
                    // creating this slice is undefined behavior.
                    let slice = slice::from_raw_parts_mut(fst.as_mut_ptr(), fst.len() + snd.len()); // TODO: u8 arrays cast into u16
                    let ptr = slice as *mut [i8];
                    let val = ptr.as_uninit_slice_mut();
                    println!("{:?}", val);
                }
                true
            }
            let mut a = 1i8;
            let mut b = 2i8;
            join_slices(slice::from_mut(&mut a), slice::from_mut(&mut b));
            true
        }
        6 => {
            unsafe{
                let slice = {
                    let temp = &mut [T::default(); 5];
                    let ptr = temp as *mut [T];
                    ptr.as_uninit_slice_mut()
                };
                println!("{:?}", slice); // UB, but maybe not associated with the Alive
            }
            true
        }
        _ => { // Path 0: Normal case
            unsafe { slice_ptr.as_uninit_slice_mut();
            println!("{:?}", slice_ptr);
            }
            true
        },
    }
}

fn main() {
    // Create a valid slice for testing
    let mut arr = [0i32, 0, 0, 0, 0];
    let slice_ptr = &mut arr as *mut [i32];
    
    // Test different paths
    let res = case_as_uninit_slice_mut1(slice_ptr, 5, 6, 1);
    println!("Result: {:?}", res);
}
