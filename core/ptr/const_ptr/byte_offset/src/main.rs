fn case_byte_offset1<T>(ptr: *const T, offset: isize) -> *const T { // InBounded
    unsafe { ptr.byte_offset(offset) }
}

fn case_byte_offset2<T>(ptr: *const T, offset: isize) -> *const T { // ValidNum
    if (offset as usize) * size_of::<T>() < isize::MAX as usize {
        ptr
    } else {
        unsafe { ptr.byte_offset(isize::MIN) }
    }
}
fn test_true_InBounded() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u32 = arr.as_ptr();
    let p2 = case_byte_offset1(p, 8);
}

fn main() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u32 = arr.as_ptr();
    let p2 = case_byte_offset1(p, isize::MIN);
}
