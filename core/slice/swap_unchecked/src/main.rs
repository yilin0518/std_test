#![feature(slice_swap_unchecked)]

fn case_swap_unchecked1<'a>(input: u32) -> bool {
    let mut arr = [1u32, 2, 3, 4, 5];
    let slice = &mut arr[..];
    match input {
        3 => {
            unsafe { slice.swap_unchecked(1, 3) }; // InBounded
            assert_eq!(arr[1], 4);
            assert_eq!(arr[3], 2);
        }
        5 => {
            unsafe { slice.swap_unchecked(1, 5) }; // Out of Bounds - undefined behavior
        }
        1 => {
            unsafe { slice.swap_unchecked(5, 1) }; // Out of Bounds - undefined behavior
        }
        _ => {}
    };
    true
}

fn main() {
    case_swap_unchecked1(5);
}
