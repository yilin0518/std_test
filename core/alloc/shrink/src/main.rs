#![feature(allocator_api)]
use std::alloc::{alloc, dealloc, Layout, Allocator, Global};
use std::ptr::NonNull;

// Unsound function that violates different SPs based on paths
fn case_shrink1(path: u32) -> bool {
    match path {
        0 => {
            // Path 0: Normal case - proper shrink
            unsafe {
                let old_layout = Layout::from_size_align(1024, 8).unwrap();
                let new_layout = Layout::from_size_align(512, 8).unwrap();
                
                let ptr = alloc(old_layout);
                if ptr.is_null() {
                    return false;
                }
                
                let non_null_ptr = NonNull::new(ptr).unwrap();
                
                // Shrink the allocation - this should be safe
                match Global.shrink(non_null_ptr, old_layout, new_layout) {
                    Ok(new_ptr) => {
                        // Clean up the shrunk allocation
                        Global.deallocate(new_ptr.cast(), new_layout);
                    }
                    Err(_) => {
                        // If shrink fails, deallocate the original
                        dealloc(ptr, old_layout);
                    }
                }
            }
            true
        }
        1 => {
            // Path 1: Allocated violation - use after free
            unsafe {
                let old_layout = Layout::from_size_align(1024, 8).unwrap();
                let new_layout = Layout::from_size_align(512, 8).unwrap();
                
                let ptr = alloc(old_layout);
                if ptr.is_null() {
                    return false;
                }
                
                let non_null_ptr = NonNull::new(ptr).unwrap();
                
                // Deallocate the memory before shrinking
                dealloc(ptr, old_layout);
                
                // Now try to shrink - this violates Allocated SP
                // The memory has been deallocated, so this is UB
                let _ = Global.shrink(non_null_ptr, old_layout, new_layout);
            }
            true
        }
        2 => {
            // Path 2: ValidNum violation - new size larger than old size
            unsafe {
                let old_layout = Layout::from_size_align(512, 8).unwrap();
                let new_layout = Layout::from_size_align(1024, 8).unwrap(); // Larger size
                
                let ptr = alloc(old_layout);
                if ptr.is_null() {
                    return false;
                }
                
                let non_null_ptr = NonNull::new(ptr).unwrap();
                
                // Try to shrink with larger size - this violates ValidNum SP
                // new_layout.size() must be <= old_layout.size()
                let _ = Global.shrink(non_null_ptr, old_layout, new_layout);
                
                // Clean up
                dealloc(ptr, old_layout);
            }
            true
        }
        3 => {
            // Path 3: Layout violation - old_layout doesn't fit the memory block
            unsafe {
                // Allocate memory with one layout
                let actual_layout = Layout::from_size_align(1024, 8).unwrap();
                let ptr = alloc(actual_layout);
                if ptr.is_null() {
                    return false;
                }
                
                let non_null_ptr = NonNull::new(ptr).unwrap();
                
                // But claim it was allocated with a different layout that doesn't fit
                let wrong_old_layout = Layout::from_size_align(512, 8).unwrap(); // Larger than actual
                let new_layout = Layout::from_size_align(512, 8).unwrap();
                
                // Try to shrink with wrong old_layout - this violates Layout SP
                // old_layout must fit the actual memory block
                let _ = Global.shrink(non_null_ptr, wrong_old_layout, new_layout);
                
                // Clean up
                dealloc(ptr, actual_layout);
            }
            true
        }
        _ => {
            // Default case
            true
        }
    }
}

// Test functions for different scenarios
fn test_true_allocated() {
    println!("Testing true Allocated case...");
    let result = case_shrink1(0);
    println!("Result: {}", result);
}

fn test_false_allocated() {
    println!("Testing false Allocated case - use after free...");
    let result = case_shrink1(1);
    println!("Result: {}", result);
}

fn test_false_validnum() {
    println!("Testing false ValidNum case - invalid size...");
    let result = case_shrink1(2);
    println!("Result: {}", result);
}

fn test_false_layout() {
    println!("Testing false Layout case - old_layout doesn't fit...");
    let result = case_shrink1(3);
    println!("Result: {}", result);
}


fn main() {
    // Test different paths
    test_true_allocated();
    //test_false_allocated();
    //test_false_validnum();
    test_false_layout(); 
}
