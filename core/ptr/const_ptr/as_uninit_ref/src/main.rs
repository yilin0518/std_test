#![feature(ptr_as_uninit)]

fn test_true_Null() -> bool {
    let p: *const u32 = std::ptr::null();
    let r = unsafe { p.as_uninit_ref() };
    if r.is_some() {
        return false;
    } else {
        return true;
    }
}

fn test_true_ValidPtr2Ref() {
    let x = 42u32;
    let p: *const u32 = &x;
    let r = unsafe { p.as_uninit_ref() };
    assert!(r.is_some());
}

fn test_false_ValidPtr2Ref_Init() {
    use std::mem::MaybeUninit;
    let x: MaybeUninit<u32> = MaybeUninit::uninit();
    let p = x.as_ptr();
    // Init(p, T, 1) violated - memory not initialized
    let r = unsafe { p.as_uninit_ref() };
    // undefined behavior
    println!("{:?}", r);
}

fn test_false_ValidPtr2Ref_Align() {
    let mut buf = [0u8; 8];
    let p = unsafe { buf.as_ptr().add(1) as *const u32 };
    // Align(p, T) violated - pointer not aligned
    let r = unsafe { p.as_uninit_ref() };
    // undefined behavior
    println!("{:?}", r);
}

fn test_false_ValidPtr2Ref_Alias() {
    let mut x = 42u32;
    let mut p: *mut u32 = &mut x as *mut u32;
    let r1 = unsafe { p.as_uninit_ref().unwrap() };
    // Alias(p, 0) violated - creating multiple aliases to same data
    // This violates Rust's exclusive mutability principle
    unsafe { *p = 43; }
    println!("{:?}", r1);
}

fn main() {
    // test_true_Null();
    // test_true_ValidPtr2Ref();
    // test_false_ValidPtr2Ref_Init();
    // test_false_ValidPtr2Ref_Align();
    // test_false_ValidPtr2Ref_Alias();
}
