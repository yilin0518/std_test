use std::mem::ManuallyDrop;
use std::alloc::{alloc, dealloc, Layout};
use std::fmt::Debug;

#[repr(C)]
#[derive(Debug)]
struct NoCopy {
    x: i32,
    y: Vec<i32>,
}



// Unsound function that violates Allocated SP based on different paths
fn case_manually_drop1(
    path: u32,
) -> bool {
    match path {
         0 => {
            // Path 0: Normal case - proper allocation
            unsafe {
                let layout = Layout::new::<NoCopy>();
                let ptr = alloc(layout);
                if ptr.is_null() {
                    return false;
                }
                
                // Write the value to allocated memory
                std::ptr::write(ptr as *mut NoCopy, NoCopy{x: 1, y: vec![1, 2, 3]});
                
                // Create ManuallyDrop with properly allocated memory
                let value = std::ptr::read(ptr as *const NoCopy);
                let mut md = ManuallyDrop::new(value);
                
                // Drop the value - this should be safe
                ManuallyDrop::drop(&mut md);
                
                // Clean up the allocated memory
                dealloc(ptr, layout);
            }
            true
        }
        1 => {
            // Path 1: Allocated violation - use after free
            unsafe {
                let layout = Layout::new::<NoCopy>();
                let ptr = alloc(layout);
                if ptr.is_null() {
                    return false;
                }
                
                // Write the value to allocated memory
                std::ptr::write(ptr as *mut NoCopy, NoCopy{x: 1, y: vec![1, 2, 3]});
                
                // Create ManuallyDrop with allocated memory
                let value = std::ptr::read(ptr as *const NoCopy);
                let mut md = ManuallyDrop::new(value);
                
                // The memory has been deallocated
                ManuallyDrop::drop(&mut md);

                // Use after free, UB
                md.y[0] = 1;
            }
            true
        }
        2 => {
            // Path 2: Double Drop 
            unsafe {
                let layout = Layout::new::<NoCopy>();
                let ptr = alloc(layout);
                if ptr.is_null() {
                    return false;
                }
                
                // Write the value to allocated memory
                std::ptr::write(ptr as *mut NoCopy, NoCopy{x: 1, y: vec![1, 2, 3]});
                
                // Create ManuallyDrop with allocated memory
                let value = std::ptr::read(ptr as *const NoCopy);
                let mut md = ManuallyDrop::new(value);
                
                ManuallyDrop::drop(&mut md);

                // Double Drop, UB
                ManuallyDrop::drop(&mut md);
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
    let result = case_manually_drop1(0);
    println!("Result: {}", result);
}

fn test_false_allocated_use_after_free() {
    println!("Testing false Allocated case - use after free...");
    let result = case_manually_drop1(1);
    println!("Result: {}", result);
}

fn test_false_allocated_double_drop() {
    println!("Testing false Allocated case - double drop...");
    let result = case_manually_drop1(2);
    println!("Result: {}", result);
}



fn main() {
    // Test different paths
    test_true_allocated();
    test_false_allocated_use_after_free();
    test_false_allocated_double_drop();
}
