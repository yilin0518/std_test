fn case_offset1<T>(ptr: *const T, offset: isize) -> *const T { // InBounded
    unsafe { ptr.offset(offset) }
}

fn case_offset2<T>(ptr: *const T, offset: isize) -> *const T { // ValidNum
    if (offset as usize) * size_of::<T>() < isize::MAX as usize {
        ptr
    } else {
        unsafe { ptr.offset(offset) }
    }
}

fn main() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u32 = arr.as_ptr();
    let p2 = case_offset2(p, isize::MAX/2);
}
