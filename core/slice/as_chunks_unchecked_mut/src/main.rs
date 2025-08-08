fn case_as_chunks_unchecked_mut1<'a>(input: u32) -> bool {
    let mut arr = [1u32, 2, 3, 4, 5, 6];
    let slice = &mut arr[..];
    match input {
        0 => {
            let chunks = unsafe { slice.as_chunks_unchecked_mut::<0>() }; // Invalid chunk size - slice length not divisible by 0
        }
        2 => {
            let chunks = unsafe { slice.as_chunks_unchecked_mut::<2>() }; // ValidNum - chunk size divides slice length
            chunks[0][0] = 10;
            chunks[1][1] = 20;
            assert_eq!(chunks, &[[10, 2], [3, 20], [5, 6]]);
        }
        4 => {
            let chunks = unsafe { slice.as_chunks_unchecked_mut::<4>() }; // Invalid chunk size - slice length not divisible by 3
        }
        _ => {}
    };
    true
}

fn main() {
    case_as_chunks_unchecked_mut1(2);
}
