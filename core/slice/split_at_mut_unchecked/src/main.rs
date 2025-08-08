fn case_split_at_mut_unchecked1<'a>(input: u32) -> bool {
    let mut arr = [1u32, 2, 3, 4, 5];
    let slice = &mut arr[..];
    match input {
        2 => {
            let (left, right) = unsafe { slice.split_at_mut_unchecked(2) }; // InBounded
            left[0] = 10;
            right[0] = 20;
            assert_eq!(arr[0], 10);
            assert_eq!(arr[2], 20);
        }
        6 => {
            let _result = unsafe { slice.split_at_mut_unchecked(6) }; // Out of Bounds - undefined behavior
        }
        _ => {}
    };
    true
}

fn main() {
    case_split_at_mut_unchecked1(2);
}
