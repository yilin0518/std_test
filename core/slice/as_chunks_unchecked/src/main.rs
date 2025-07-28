fn test_true_ValidNum() {
    let arr = [1u32, 2, 3, 4, 5, 6];
    let slice = &arr[..];
    let chunks = unsafe { slice.as_chunks_unchecked::<2>() }; // ValidNum - chunk size divides slice length
    assert_eq!(chunks, &[[1, 2], [3, 4], [5, 6]]);
}

fn test_false_ValidNum_zero() {
    let arr = [1u32, 2, 3, 4, 5];
    let slice = &arr[..];
    let _chunks = unsafe { slice.as_chunks_unchecked::<0>() }; // Invalid chunk size - slice length not divisible by 2
}

fn test_false_ValidNum_not_divisible() {
    let arr = [1u32, 2, 3, 4, 5];
    let slice = &arr[..];
    let _chunks = unsafe { slice.as_chunks_unchecked::<3>() }; // Invalid chunk size - slice length not divisible by 3
}


fn main() {
    // test_true_ValidNum();
    // test_false_ValidNum_zero();
    // test_false_ValidNum_not_divisible();
}
