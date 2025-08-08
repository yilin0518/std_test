fn case_split_at_unchecked1<'a>(input: u32) -> bool {
    let arr = [1u32, 2, 3, 4, 5];
    let slice = &arr[..];
    match input {
        2 => {
            let (left, right) = unsafe { slice.split_at_unchecked(2) }; // InBounded
            assert_eq!(left, &[1, 2]);
            assert_eq!(right, &[3, 4, 5]);
        }
        6 => {
            let _result = unsafe { slice.split_at_unchecked(6) }; // Out of Bounds - undefined behavior
        }
        _ => {}
    };
    true
}

fn main() {
    case_split_at_unchecked1(6);
}
