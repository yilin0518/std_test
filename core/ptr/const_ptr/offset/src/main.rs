fn test_true_InBounded() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u32 = arr.as_ptr();
    let p2 = unsafe { p.offset(2) }; // InBounded
    assert_eq!(unsafe { *p2 }, 3);
}

fn test_false_InBounded() {
    let arr = [1u32, 2, 3];
    let p: *const u32 = arr.as_ptr();
    let p2 = unsafe { p.offset(5) }; // Out of Bounds
    let _ = unsafe { *p2 }; // undefined behavior
}

fn test_true_ValidNum() {
    let arr = [1u32, 2, 3, 4, 5]; // ValidNum
    let p: *const u32 = arr.as_ptr();
    let count: isize = 2;
    let p2 = unsafe { p.offset(count) };
    assert_eq!(unsafe { *p2 }, 3);
}

fn test_false_ValidNum() {
    let arr = [1u32, 2, 3]; // ValidNum
    let p: *const u32 = arr.as_ptr();
    let count: isize = isize::MAX; // invalid count
    let p2 = unsafe { p.offset(count) };
    let _ = unsafe { *p2 }; // undefined behavior
}

fn main() {
    // test_true_InBounded();
    // test_false_InBounded();
    // test_true_ValidNum();
    test_false_ValidNum();
}
