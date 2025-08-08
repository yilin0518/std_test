fn case_byte_sub1<T>(ptr: *const T, offset: usize) -> *const T { // InBounded
    unsafe { ptr.byte_sub(offset) }
}

fn case_byte_sub2<T>(ptr: *const T, offset: usize) -> *const T { // ValidNum
    if offset < isize::MAX as usize {
        ptr
    } else {
        unsafe { ptr.byte_sub(offset) }
    }
}   


fn main() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u32 = arr.as_ptr();
    let p2 = case_byte_sub2(p, isize::MAX as usize + 1);
}
