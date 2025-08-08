use std::os::fd::{FromRawFd, OwnedFd, AsRawFd};
use std::os::unix::io::RawFd;

// Unsound function that violates different SPs based on paths
fn case_from_raw_fd1(path: u32) -> Option<OwnedFd> {
    match path {
        0 => {
            // Path 0: Normal case - safe from_raw_fd
            // Open a file and use its fd
            match std::fs::File::open("test.txt") {
                Ok(file) => {
                    let fd = file.as_raw_fd();
                    println!("Opened test.txt with fd: {}", fd);
                    // This is safe because the file is open
                    // We need to forget the original file to avoid double close
                    let owned_fd = unsafe { OwnedFd::from_raw_fd(fd) };
                    println!("Successfully created OwnedFd from raw fd");
                    // Prevent the original file from being dropped
                    std::mem::forget(file);
                    // Return the owned_fd to prevent it from being dropped immediately
                    Some(owned_fd)
                }
                Err(e) => {
                    println!("Failed to open test.txt: {}", e);
                    return None;
                }
            }
        }
        1 => {
            // Path 1: Opened violation - invalid fd (-1)
            let invalid_fd: RawFd = -1;
            println!("Attempting to create OwnedFd from invalid fd: {}", invalid_fd);
            // This violates Opened SP - fd -1 is not a valid open file descriptor
            let owned_fd = unsafe { OwnedFd::from_raw_fd(invalid_fd) };
            Some(owned_fd)
        }
        2 => {
            // Path 2: Opened violation - never opened fd
            let never_opened_fd: RawFd = 999; // Some arbitrary fd number
            println!("Attempting to create OwnedFd from never opened fd: {}", never_opened_fd);
            // This violates Opened SP - this fd was never opened
            let owned_fd = unsafe { OwnedFd::from_raw_fd(never_opened_fd) };
            println!("OwnedFd: {:?}", owned_fd);
            Some(owned_fd)
        }
        3 => {
            // Path 3: Opened violation - fd that was closed
            let fd: RawFd;
            {
                match std::fs::File::open("test.txt") {
                    Ok(file) => {
                        fd = file.as_raw_fd();
                        println!("Opened test.txt with fd: {}", fd);
                        // Close the file but keep the fd number
                        drop(file);
                    }
                    Err(e) => {
                        println!("Failed to open test.txt: {}", e);
                        return None;
                    }
                }
            }
            
            println!("Attempting to create OwnedFd from closed fd: {}", fd);
            // This violates Opened SP - the file has been closed
            let owned_fd = unsafe { OwnedFd::from_raw_fd(fd) };
            println!("OwnedFd: {:?}", owned_fd);
            Some(owned_fd)
        }
        _ => {
            // Default case - safe usage
            match std::fs::File::open("test.txt") {
                Ok(file) => {
                    let fd = file.as_raw_fd();
                    let owned_fd = unsafe { OwnedFd::from_raw_fd(fd) };
                    println!("Default case: successfully created OwnedFd");
                    // Prevent the original file from being dropped
                    std::mem::forget(file);
                    Some(owned_fd)
                }
                Err(e) => {
                    println!("Default case failed: {}", e);
                    None
                }
            }
        }
    }
}

fn main() {
    case_from_raw_fd1(3);
}
