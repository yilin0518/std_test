#[allow(dead_code)]
fn case_offset_from1(path: u32) -> Option<isize> {
    match path {
        0 => {
            // Path 0: Normal case (sound) - same allocation, difference is multiple of size_of::<T>()
            let a = [10i32, 20, 30, 40, 50];
            let p1: *const i32 = &a[1];
            let p2: *const i32 = &a[4];
            let diff = unsafe { p2.offset_from(p1) };
            println!("Path0 diff = {} (expected 3)", diff);
            Some(diff)
        }
        1 => {
            // Path 1: InBounded violation - pointers from different allocations
            // Unsound: no check that pointers originate from the same allocation
            let p1 = Box::into_raw(Box::new(0u8)) as *const u8;
            let p2 = Box::into_raw(Box::new(1u8)) as *const u8;
            println!("Path1 calling offset_from on pointers from different allocations");
            // SAFETY: This violates the safety contract (not same allocation)
            let diff = unsafe { (p2 as *const u8).offset_from(p1 as *const u8) };
            // Leak the allocations to avoid double-free in case of non-UB runs
            std::mem::forget(unsafe { Box::from_raw(p1 as *mut u8) });
            std::mem::forget(unsafe { Box::from_raw(p2 as *mut u8) });
            Some(diff)
        }
        2 => {
            // Path 2: ValidNum violation - distance in bytes is NOT a multiple of size_of::<T>()
            // Construct two pointers into the same byte buffer that differ by 1 byte,
            // then view them as *const u16 and compute offset_from::<u16>.
            let buf: [u8; 4] = [1, 2, 3, 4];
            let base = buf.as_ptr();
            let p1 = base as *const u16;                // address aligned to buf start
            let p2 = &buf[1] as *const u8 as *const u16; // address + 1 byte
            println!("Path2 calling offset_from::<u16> on pointers 1 byte apart");
            // SAFETY: Violates the requirement that the distance in bytes is a multiple of size_of::<T>()
            let diff = unsafe { (p2 as *const u16).offset_from(p1 as *const u16) };
            Some(diff)
        }
        3 => {
            // Path 3: !Size violation - using a Zero-Sized Type (ZST)
            // offset_from panics for ZST; the function is unsound because it does not prevent T from being ZST
            let arr: [(); 2] = [(), ()];
            let p1: *const () = &arr[0];
            let p2: *const () = &arr[1];
            println!("Path3 calling offset_from::<()> which will panic due to ZST");
            let diff = unsafe { p2.offset_from(p1) }; // will panic
            Some(diff)
        }
        _ => {
            // Default: behave like normal case
            let a = [1i64, 2, 3, 4, 5, 6];
            let p1: *const i64 = &a[0];
            let p2: *const i64 = &a[5];
            let diff = unsafe { p2.offset_from(p1) };
            println!("Default diff = {} (expected 5)", diff);
            Some(diff)
        }
    }
}

fn main() {
    let _ = case_offset_from1(3);
    // let _ = case_offset_from1(1); // InBounded violation
    // let _ = case_offset_from1(2); // ValidNum violation
    // let _ = case_offset_from1(3); // !Size violation (panic)
}
