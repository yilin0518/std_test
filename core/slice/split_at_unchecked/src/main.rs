fn test_true_InBounded() {
    let arr = [1u32, 2, 3, 4, 5];
    let slice = &arr[..];
    let (left, right) = unsafe { slice.split_at_unchecked(2) }; // InBounded
    assert_eq!(left, &[1, 2]);
    assert_eq!(right, &[3, 4, 5]);
}

fn test_false_InBounded() {
    let arr = [1u32, 2, 3];
    let slice = &arr[..];
    let _result = unsafe { slice.split_at_unchecked(5) }; // Out of Bounds - undefined behavior
}

fn main() {
    // test_true_InBounded();
    // test_false_InBounded();
}
