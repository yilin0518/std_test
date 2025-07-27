fn test_true_InBounded() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u8 = arr.as_ptr().cast().add(16); // array[3]
    let p2 = unsafe { p.byte_sub(8) }; // InBounded, offset 2 * sizeof(u32)
    let p2_u32 = unsafe { p2.cast::<u32>() };
    assert_eq!(unsafe { *p2_u32 }, 3);
}

fn test_false_InBounded() {
    let arr = [1u32, 2, 3];
    let p: *const u8 = arr.as_ptr().cast().add(8); // array[1]
    let p2 = unsafe { p.byte_sub(20) }; // Out of Bounds
    let _ = unsafe { *p2 }; // undefined behavior
}

fn test_true_ValidNum() {
    let arr = [1u32, 2, 3, 4, 5]; // ValidNum
    let p: *const u8 = arr.as_ptr().cast().add(16);
    let count: usize = 8; // 2 * sizeof(u32)
    let p2 = unsafe { p.byte_sub(count) };
    let p2_u32 = unsafe { p2.cast::<u32>() };
    assert_eq!(unsafe { *p2_u32 }, 3);
}

fn test_false_ValidNum() {
    let arr = [u32::MAX, 2, 3]; // ValidNum
    let p: *const u8 = arr.as_ptr().cast().add(8);
    let count: usize = 0;
    let p2 = unsafe { p.byte_sub(count) };
    let p2_u32 = unsafe { p2.cast::<u32>() };
    let _ = unsafe { *p2_u32 + 1 }; // Out of Bounds
}

fn main() {
    // test_true_InBounded();
    // test_false_InBounded();
    // test_true_ValidNum();
    // test_false_ValidNum();
}
