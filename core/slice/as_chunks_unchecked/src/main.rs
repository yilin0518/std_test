fn case_as_chunks_unchecked1<'a>(input: u32) -> bool {
    let arr = [1u32, 2, 3, 4, 5, 6];
    let slice = &arr[..];
    match input {
        0 => {
            let chunks = unsafe { slice.as_chunks_unchecked::<0>() }; // Invalid chunk size - slice length not divisible by 0
        }
        2 => {
            let chunks = unsafe { slice.as_chunks_unchecked::<2>() }; // ValidNum - chunk size divides slice length
            assert_eq!(chunks, &[[1, 2], [3, 4], [5, 6]]);
        }
        4 => {
            let chunks = unsafe { slice.as_chunks_unchecked::<4>() }; // Invalid chunk size - slice length not divisible by 3
        }
        _ => {}
    };
    true
}

fn main() {
    let res = case_as_chunks_unchecked1(4);
}
