fn test_true_InBounded() {
    let mut arr = [1u32, 2, 3, 4, 5];
    let slice = &mut arr[..];
    let element = unsafe { slice.get_unchecked_mut(2) }; // InBounded
    *element = 10;
    assert_eq!(arr[2], 10);
}

fn test_false_InBounded() {
    let mut arr = [1u32, 2, 3];
    let slice = &mut arr[..];
    let _element = unsafe { slice.get_unchecked_mut(5) }; // Out of Bounds - undefined behavior
}

fn main() {
    // test_true_InBounded();
    // test_false_InBounded();
}
