fn case_get_unchecked_mut1<'a>(input: u32) -> bool {
    let mut arr = [1u32, 2, 3, 4, 5];
    let slice = &mut arr[..];
    match input {
        2 => {
            let element = unsafe { slice.get_unchecked_mut(2) }; // InBounded
            *element = 10;
            assert_eq!(arr[2], 10);
        }
        5 => {
            let element = unsafe { slice.get_unchecked_mut(5) }; // Out of Bounds - undefined behavior
        }
        _ => {}
    };
    true
}

fn main() {
    case_get_unchecked_mut1(5);
}
