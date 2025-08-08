#![feature(allocator_api)]
use std::alloc::{alloc, dealloc, Allocator, Global, Layout};

fn case_from_raw1(val: i32) -> bool {
    if val > 0 {
        // Normal case - use heap allocated pointer
        let layout = Layout::new::<u32>();
        let ptr = unsafe { alloc(layout) as *mut u32 };
        if ptr.is_null() {
            return false;
        }
        unsafe {
            std::ptr::write(ptr, 42u32);
        }
        let box1 = unsafe { Box::from_raw(ptr) };
        println!("Normal case: {:?}", box1);
    } else {
        // Ownning Violated
        let layout = Layout::new::<u32>();
        let ptr = unsafe { alloc(layout) as *mut u32 };
        if ptr.is_null() {
            return false;
        }
        unsafe {
            std::ptr::write(ptr, 42u32);
        }
        let box1 = unsafe { Box::from_raw(ptr) };
        let box2 = unsafe { Box::from_raw(ptr) };
        println!("Normal case: {:?}", box1);
    }
    true
}

fn case_from_raw2(val: i32) -> bool {
    match val {
        0 => {
            // Path 0: Normal case - use heap allocated pointer
            let layout = Layout::new::<u32>();
            let ptr = unsafe { alloc(layout) as *mut u32 };
            if ptr.is_null() {
                return false;
            }
            unsafe {
                std::ptr::write(ptr, 42u32);
            }
            let box1 = unsafe { Box::from_raw(ptr) };
            println!("Normal case: {:?}", box1);
        }
        1 => {
            // Path 1: Allocated violation - use after free
            let layout = Layout::new::<u32>();
            let ptr = unsafe { alloc(layout) as *mut u32 };
            if ptr.is_null() {
                return false;
            }
            unsafe {
                std::ptr::write(ptr, 42u32);
                dealloc(ptr as *mut u8, layout); // Deallocate first
            }
            let box1 = unsafe { Box::from_raw(ptr) }; // UB: use after free
            println!("Use after free: {:?}", box1);
        }
        2 => {
            // Path 2: Allocated violation - use stack pointer
            let mut x = 42u32;
            let ptr = &mut x as *mut u32;
            let box1 = unsafe { Box::from_raw(ptr) }; // UB: stack pointer used with Box::from_raw
            println!("{:?}", box1);
        }
        _ => {
            // Default: normal case
            let layout = Layout::new::<u32>();
            let ptr = unsafe { alloc(layout) as *mut u32 };
            if ptr.is_null() {
                return false;
            }
            unsafe {
                std::ptr::write(ptr, 42u32);
            }
            let box1 = unsafe { Box::from_raw(ptr) };
            println!("Default case: {:?}", box1);
        }
    };
    true
}
fn main() {
    //case_from_raw1(0);  
    case_from_raw2(2);  
}
