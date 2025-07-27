fn test_true_InBounded() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u32 = arr.as_ptr().add(4); // array[4]
    let p2 = unsafe { p.sub(2) }; // InBounded
    assert_eq!(unsafe { *p2 }, 3);
}

fn test_false_InBounded() {
    let arr = [1u32, 2, 3];
    let p: *const u32 = arr.as_ptr().add(2); // array[2]
    let p2 = unsafe { p.sub(5) }; // Out of Bounds
    let _ = unsafe { *p2 }; // undefined behavior
}

fn test_true_ValidNum() {
    let arr = [1u32, 2, 3, 4, 5]; // ValidNum
    let p: *const u32 = arr.as_ptr().add(4);
    let count: usize = 2;
    let p2 = unsafe { p.sub(count) };
    assert_eq!(unsafe { *p2 }, 3);
}

fn test_false_ValidNum() {
    let arr = [u32::MAX, 2, 3]; // ValidNum
    let p: *const u32 = arr.as_ptr().add(2);
    let count: usize = 0;
    let p2 = unsafe { p.sub(count) };
    let _ = unsafe { *p2 + 1 }; // Out of Bounds
}

fn main() {
    // test_true_InBounded();
    // test_false_InBounded();
    // test_true_ValidNum();
    // test_false_ValidNum();
}
