#![feature(slice_swap_unchecked)]

fn test_true_InBounded() {
    let mut arr = [1u32, 2, 3, 4, 5];
    let slice = &mut arr[..];
    unsafe { slice.swap_unchecked(1, 3) }; // InBounded
    assert_eq!(arr[1], 4);
    assert_eq!(arr[3], 2);
}

fn test_false_InBounded_right() {
    let mut arr = [1u32, 2, 3];
    let slice = &mut arr[..];
    unsafe { slice.swap_unchecked(1, 5) }; // Out of Bounds - undefined behavior
}

fn test_false_InBounded_left() {
    let mut arr = [1u32, 2, 3];
    let slice = &mut arr[..];
    unsafe { slice.swap_unchecked(5, 1) }; // Out of Bounds - undefined behavior
}

fn main() {
    // test_true_InBounded();
     test_false_InBounded_right();
    //test_false_InBounded_left();
}
