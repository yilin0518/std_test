fn case_sub1<T>(ptr: *const T, offset: usize) -> *const T {
    unsafe { ptr.sub(offset) }
}

fn case_sub2<T>(ptr: *const T, offset: usize) -> *const T {
    if offset * size_of::<T>() < isize::MAX as usize {
        ptr
    } else {
        unsafe { ptr.sub(offset) }
    }
}

fn main() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u32 = arr.as_ptr();
    let p2 = case_sub2(p, (isize::MAX/2) as usize);
}
