fn test_true_InBounded() {
    let mut arr = [1u32, 2, 3, 4, 5];
    let slice = &mut arr[..];
    let (left, right) = unsafe { slice.split_at_mut_unchecked(2) }; // InBounded
    left[0] = 10;
    right[0] = 20;
    assert_eq!(arr[0], 10);
    assert_eq!(arr[2], 20);
}

fn test_false_InBounded() {
    let mut arr = [1u32, 2, 3];
    let slice = &mut arr[..];
    let _result = unsafe { slice.split_at_mut_unchecked(5) }; // Out of Bounds - undefined behavior
}

fn main() {
    // test_true_InBounded();
     test_false_InBounded();
}
