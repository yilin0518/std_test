fn case_byte_add1<T>(ptr: *const T, offset: usize) -> *const T { // InBounded
    unsafe { ptr.byte_add(offset) }
}

fn case_byte_add2<T>(ptr: *const T, offset: usize) -> *const T { // ValidNum
    if offset < isize::MAX as usize {
        ptr
    } else {
        unsafe { ptr.byte_add(offset) }
    }
}


fn main() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u32 = arr.as_ptr();
    let p2 = case_byte_add2(p, usize::MAX);
}
