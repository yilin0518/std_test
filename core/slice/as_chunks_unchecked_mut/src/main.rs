fn test_true_ValidNum() {
    let mut arr = [1u32, 2, 3, 4, 5, 6];
    let slice = &mut arr[..];
    let chunks = unsafe { slice.as_chunks_unchecked_mut::<2>() }; // ValidNum - chunk size divides slice length
    chunks[0][0] = 10;
    chunks[1][1] = 20;
    assert_eq!(chunks, &[[10, 2], [3, 20], [5, 6]]);
}

fn test_false_ValidNum_zero() {
    let mut arr = [1u32, 2, 3, 4, 5];
    let slice = &mut arr[..];
    let _chunks = unsafe { slice.as_chunks_unchecked_mut::<0>() }; // Invalid chunk size - slice length not divisible by 0
}

fn test_false_ValidNum_not_divisible() {
    let mut arr = [1u32, 2, 3, 4, 5];
    let slice = &mut arr[..];
    let _chunks = unsafe { slice.as_chunks_unchecked_mut::<3>() }; // Invalid chunk size - slice length not divisible by 3
}

fn main() {
    // test_true_ValidNum();
    // test_false_ValidNum_zero();
    // test_false_ValidNum_not_divisible();
}
