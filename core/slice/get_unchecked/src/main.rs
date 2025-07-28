fn test_true_InBounded() {
    let arr = [1u32, 2, 3, 4, 5];
    let slice = &arr[..];
    let element = unsafe { slice.get_unchecked(2) }; // InBounded
    assert_eq!(*element, 3);
}

fn test_false_InBounded() {
    let arr = [1u32, 2, 3];
    let slice = &arr[..];
    let _element = unsafe { slice.get_unchecked(5) }; // Out of Bounds - undefined behavior
}

fn main() {
   // test_true_InBounded();
   // test_false_InBounded();
}
