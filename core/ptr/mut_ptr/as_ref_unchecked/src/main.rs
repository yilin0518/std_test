fn test_true_ValidPtr2Ref() {
    let mut x = 42u32;
    let p: *mut u32 = &mut x;
    let r = unsafe { p.as_ref_unchecked() };
    assert_eq!(r, &42u32);
}

fn test_false_ValidPtr2Ref_Init() {
    use std::mem::MaybeUninit;
    let mut x: MaybeUninit<u32> = MaybeUninit::uninit();
    let p = x.as_mut_ptr();
    // Init(p, T, 1) violated - memory not initialized
    let r = unsafe { p.as_ref_unchecked() };
    // undefined behavior
    println!("{:?}", r);
}

fn test_false_ValidPtr2Ref_Align() {
    let mut buf = [0u8; 8];
    let p = unsafe { buf.as_ptr().add(1) as *mut u32 };
    // Align(p, T) violated - pointer not aligned
    let r = unsafe { p.as_ref_unchecked() };
    // undefined behavior
    println!("{:?}", r);
}

fn test_false_ValidPtr2Ref_Alias() {
    let mut x = 42u32;
    let p: *mut u32 = &mut x as *mut u32;
    let r1 = unsafe { p.as_ref_unchecked() };
    // Alias(p, 0) violated - creating multiple aliases to same mutable data
    // This violates Rust's exclusive mutability principle
    unsafe { *p = 43; }
    println!("{:?}", r1);
}

fn main() {
    // test_true_ValidPtr2Ref();
    // test_false_ValidPtr2Ref_Init();
    // test_false_ValidPtr2Ref_Align();
    // test_false_ValidPtr2Ref_Alias();
}
