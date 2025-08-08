#![feature(core_intrinsics)]

use std::intrinsics;
use std::fmt::Debug;

fn case_typed_swap_nonoverlapping1<T>(path: u32, mut left: T, mut right: T) 
where
    T: Copy + Debug + Default,
{
    match path {
        0 => {
            // Path 0: Normal case - safe typed_swap_nonoverlapping
            println!("Before swap: left = {:?}, right = {:?}", left, right);
            unsafe { intrinsics::typed_swap_nonoverlapping(&mut left, &mut right) };
            println!("After swap: left = {:?}, right = {:?}", left, right);
        }
        1 => {
            // Path 1: ValidPtr violation - null pointer
            let null_ptr: *mut T = std::ptr::null_mut();
            // This violates ValidPtr SP - using null pointer
            unsafe { intrinsics::typed_swap_nonoverlapping(&mut left, null_ptr) };
        }
        2 => {
            // Path 2: ValidPtr violation - dangling pointer
            let dangling_ptr: *mut T;
            {
                let temp = T::default();
                dangling_ptr = &temp as *const T as *mut T;
            } // temp goes out of scope, dangling_ptr becomes dangling
            // This violates ValidPtr SP - using dangling pointer
            unsafe { intrinsics::typed_swap_nonoverlapping(&mut left, dangling_ptr) };
        }
        3 => {
            // Path 3: NonOverlap violation - overlapping memory regions
            let mut array: [T; 2] = unsafe { std::mem::zeroed() };
            // This violates NonOverlap SP - swapping overlapping regions
            // We create overlapping pointers by pointing to overlapping memory regions
            let ptr1 = &mut array[0] as *mut T;
            // Create a pointer that overlaps with ptr1 by offsetting by half the size of T
            let byte_ptr1 = ptr1 as *mut u8;
            let overlapping_byte_ptr = unsafe { byte_ptr1.add(std::mem::size_of::<T>() / 2) };
            let overlapping_ptr = overlapping_byte_ptr as *mut T;
            unsafe { intrinsics::typed_swap_nonoverlapping(ptr1, overlapping_ptr) };
        }
        4 => {
            // Path 4: NonOverlap violation - same pointer
            // This violates NonOverlap SP - swapping with itself
            unsafe { intrinsics::typed_swap_nonoverlapping(&mut left, &mut left) };
        }
        5 => {
            // Path 5: ValidPtr violation - unaligned pointer
            #[repr(packed)]
            #[derive(Copy, Clone, Debug, Default)]
            struct PackedStruct<T> {
                a: u8,
                b: T,  // This will be misaligned due to packed layout
            }
            
            let mut packed = PackedStruct { a: 1, b: T::default() };
            let mut normal: T = T::default();
            // This violates ValidPtr SP - b field is not properly aligned
            let b_ptr = std::ptr::addr_of_mut!(packed.b);
            unsafe { intrinsics::typed_swap_nonoverlapping(&mut normal, b_ptr) };
        }
        _ => {
            // Default case - safe swap
            unsafe { intrinsics::typed_swap_nonoverlapping(&mut left, &mut right) };
        }
    }
}

fn main() {
    // Test different paths with different types
    case_typed_swap_nonoverlapping1::<u32>(5, 1, 2);
}
