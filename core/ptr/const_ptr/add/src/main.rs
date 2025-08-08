fn case_add1<T>(ptr: *const T, offset: usize) -> *const T { // InBounded
    unsafe { ptr.add(offset) }  
}

fn case_add2<T>(ptr: *const T, offset: usize) -> *const T { // ValidNum
    if offset * size_of::<T>()  < isize::MAX as usize{
        ptr
    } else {
        unsafe { ptr.add(offset) } 
    }
}

fn main() {
    let arr = [1u32, 2, 3, 4, 5];
    let p: *const u32 = arr.as_ptr();
    let p2 = case_add2(p, (isize::MAX/2) as usize);
}
