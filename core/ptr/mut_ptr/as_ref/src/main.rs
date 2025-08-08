fn test_true_Null() {
    let p: *mut u32 = std::ptr::null_mut();
    let r = unsafe { p.as_ref() };
    assert_eq!(r, None);
}

fn case_as_ref1(val: i32) -> bool {
    match val {
        0 => {
            let mut x = 42u32;
            let p: *mut u32 = &mut x;
            let r = unsafe { p.as_ref() };
            assert_eq!(r, Some(&42u32));
        }
        1 => {
            use std::mem::MaybeUninit;
            let mut x: MaybeUninit<u32> = MaybeUninit::uninit();
            let p = x.as_mut_ptr();
            // Init(p, T, 1) violated - memory not initialized
            let r = unsafe { p.as_ref() };
            // undefined behavior
            println!("{:?}", r);
        }
        2 => {
            let mut buf = [0u16; 8];
            let p = unsafe { buf.as_ptr().add(1) as *mut u32 };
            // Align(p, T) violated - pointer not aligned
            let r = unsafe { p.as_ref() };
            // undefined behavior
            println!("{:?}", r);
        }
        3 => {
            let mut x = 42u32;
            let p: *mut u32 = &mut x as *mut u32;
            let r1 = unsafe { p.as_ref().unwrap() };
            // Alias(p, 0) violated - creating multiple aliases to same mutable data
            // This violates Rust's exclusive mutability principle
            unsafe {
                *p = 43;
            }
            println!("{:?}", r1);
        }
        _ => {}
    };
    true
}
fn test_true_ValidPtr2Ref() {
    let mut x = 42u32;
    let p: *mut u32 = &mut x;
    let r = unsafe { p.as_ref() };
    assert_eq!(r, Some(&42u32));
}

fn test_false_ValidPtr2Ref_Init() {
    use std::mem::MaybeUninit;
    let mut x: MaybeUninit<u32> = MaybeUninit::uninit();
    let p = x.as_mut_ptr();
    // Init(p, T, 1) violated - memory not initialized
    let r = unsafe { p.as_ref() };
    // undefined behavior
    println!("{:?}", r);
}

fn test_false_ValidPtr2Ref_Align() {
    let mut buf = [0u8; 8];
    let p = unsafe { buf.as_ptr().add(1) as *mut u32 };
    // Align(p, T) violated - pointer not aligned
    let r = unsafe { p.as_ref() };
    // undefined behavior
    println!("{:?}", r);
}

fn test_false_ValidPtr2Ref_Alias() {
    let mut x = 42u32;
    let p: *mut u32 = &mut x as *mut u32;
    let r1 = unsafe { p.as_ref().unwrap() };
    // Alias(p, 0) violated - creating multiple aliases to same mutable data
    // This violates Rust's exclusive mutability principle
    unsafe {
        *p = 43;
    }
    println!("{:?}", r1);
}

fn main() {
    case_as_ref1(3);
}
