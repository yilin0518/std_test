fn test_true_InBounded() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u8 = arr.as_ptr().cast();
    let p2 = unsafe { p.byte_offset(8) }; // InBounded, offset 2 * sizeof(u32)
    let p2_u32 = unsafe { p2.cast::<u32>() };
    assert_eq!(unsafe { *p2_u32 }, 3);
}

fn test_false_InBounded() {
    let arr = [1u32, 2, 3];
    let p: *const u8 = arr.as_ptr().cast();
    let p2 = unsafe { p.byte_offset(20) }; // Out of Bounds
    let _ = unsafe { *p2 }; // undefined behavior
}

fn test_true_ValidNum() {
    let arr = [1u32, 2, 3, 4, 5]; // ValidNum
    let p: *const u8 = arr.as_ptr().cast();
    let count: isize = 8; // 2 * sizeof(u32)
    let p2 = unsafe { p.byte_offset(count) };
    let p2_u32 = unsafe { p2.cast::<u32>() };
    assert_eq!(unsafe { *p2_u32 }, 3);
}

fn test_false_ValidNum() {
    let arr = [1u32, 2, 3]; // ValidNum
    let p: *const u8 = arr.as_ptr().cast();
    let count: isize = i64::MAX as isize; // invalid count
    let p2 = unsafe { p.byte_offset(count) };
    let p2_u32 = unsafe { p2.cast::<u32>() };
    let _ = unsafe { *p2_u32 }; // undefined behavior
}

fn main() {
    // test_true_InBounded();
    // test_false_InBounded();
    // test_true_ValidNum();
    test_false_ValidNum();  
}
