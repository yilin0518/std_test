fn case_get_unchecked1<'a>(input: u32) -> bool {
    let arr = [1u32, 2, 3, 4, 5];
    let slice = &arr[..];
    match input {
        2 => {
            let element = unsafe { slice.get_unchecked(2) }; // InBounded
            assert_eq!(*element, 3);
        }
        5 => {
            let element = unsafe { slice.get_unchecked(5) }; // Out of Bounds - undefined behavior
        }
        _ => {}
    };
    true
}

fn main() {
    case_get_unchecked1(2);
}
