fn test_true_InBounded() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u32 = unsafe { arr.as_ptr().add(4) }; // array[4]
    let p2 = unsafe { p.sub(2) }; // InBounded
    assert_eq!(unsafe { *p2 }, 3);
}

fn test_false_InBounded() {
    let arr = [1u32, 2, 3];
    let p: *const u32 = unsafe { arr.as_ptr().add(2) }; // array[2]
    let p2 = unsafe { p.sub(5) }; // Out of Bounds
    let _ = unsafe { *p2 }; // undefined behavior
}

fn test_true_ValidNum() {
    let arr = [1u32, 2, 3, 4, 5]; // ValidNum
    let p: *const u32 = unsafe { arr.as_ptr().add(4) };
    let count: usize = 2;
    let p2 = unsafe { p.sub(count) };
    assert_eq!(unsafe { *p2 }, 3);
}

fn test_false_ValidNum() {
    let arr = [1u32, 2, 3]; // ValidNum
    let p: *const u32 = unsafe { arr.as_ptr() };
    let count: usize = usize::MAX / 2; // invalid count
    let p2 = unsafe { p.sub(count) };
    let val = unsafe { *p2 }; // undefined behavior
    println!("val: {}", val);
}

fn main() {
    // test_true_InBounded();
    // test_false_InBounded();
    // test_true_ValidNum();
    // test_false_ValidNum();
}
