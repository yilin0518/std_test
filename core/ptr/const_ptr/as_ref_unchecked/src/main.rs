#![feature(ptr_as_ref_unchecked)]

fn case_as_ref_unchecked1<'a, T: Clone>(input: *const T, path: u32, val: T, offset: isize) -> &'a T {
    match path {
        0 => {
            unsafe { input.as_ref_unchecked() } // UnInitialized
        }
        1 => {
            let misaligned_ptr = unsafe { (input as *const u8).offset(offset) as *const T };
            unsafe { misaligned_ptr.as_ref_unchecked() } // Misaligned
        }
        2 => {
            unsafe {
                let mut x = (*input).clone();
                let ptr = &x as *const T;
                let ref_val = ptr.as_ref_unchecked();
                x = val.clone();
                ref_val // Alias
            }
        }
        _ => {
            unsafe { input.as_ref_unchecked() }
        }
    }
}
fn test_true_ValidPtr2Ref() {
    let x = 42u32;
    let p: *const u32 = &x;
    let r = unsafe { p.as_ref_unchecked() };
    assert_eq!(r, &42u32);
}

fn test_false_ValidPtr2Ref_Init() {
    use std::mem::MaybeUninit;
    let x: MaybeUninit<u32> = MaybeUninit::uninit();
    let p = x.as_ptr();
    // Init(p, T, 1) violated - memory not initialized
    let r = unsafe { p.as_ref_unchecked() };
    // undefined behavior
    println!("{:?}", r);
}

fn test_false_ValidPtr2Ref_Align() {
    let mut buf = [0u8; 8];
    let p = unsafe { buf.as_ptr().add(1) as *const u32 };
    // Align(p, T) violated - pointer not aligned
    let r = unsafe { p.as_ref_unchecked() };
    // undefined behavior
    println!("{:?}", r);
}

fn test_false_ValidPtr2Ref_Alias() {
    let mut x = 42u32;
    let mut p: *mut u32 = &mut x as *mut u32;
    let r1 = unsafe { p.as_ref_unchecked() };
    // Alias(p, 0) violated - creating multiple aliases to same data
    // This violates Rust's exclusive mutability principle
    unsafe { *p = 43; }
    println!("{:?}", r1);
}

fn main() {
    let x: *const u32 = std::ptr::null();
    let r = case_as_ref_unchecked1(x, 0, 42, 4);
    println!("{:?}", r);
}
