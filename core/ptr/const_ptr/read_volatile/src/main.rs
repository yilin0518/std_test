fn test_true_ValidPtr() {
    let x = 123u32;
    let p: *const u32 = &x;
    let y = unsafe { core::ptr::read_volatile(p) };
    assert_eq!(y, 123u32);
}

fn test_false_ValidPtr_null() {
    let p: *const u32 = std::ptr::null();
    // undefined behavior
    let _ = unsafe { core::ptr::read_volatile(p) };
}

fn test_false_ValidPtr_use_after_free() {
    let p: *const u32;
    {
        let x = 123u32;
        p = &x;
    } // x 生命周期结束，p 成为悬垂指针
    // undefined behavior - use after free
    let _ = unsafe { core::ptr::read_volatile(p) };
}

fn test_true_Aligned() {
    let x = 0u32;
    let p: *const u32 = &x;
    let y = unsafe { core::ptr::read_volatile(p) };
    assert_eq!(y, 0u32);
}

fn test_false_Aligned_byte_offset() {
    let mut buf = [0u8; 8];
    let p = unsafe { buf.as_ptr().add(1) as *const u32 };
    // undefined behavior
    let _ = unsafe { core::ptr::read_volatile(p) };
}

fn test_false_Aligned_dynamic_alloc() {
    let layout = std::alloc::Layout::from_size_align(8, 4).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    unsafe { std::ptr::write_bytes(ptr, 0, 8); }
    let p = unsafe { ptr.add(1) as *const u32 };
    // undefined behavior
    let _ = unsafe { core::ptr::read_volatile(p) };
    unsafe { std::alloc::dealloc(ptr, layout); }
}

fn test_false_Aligned_struct_field() {
    #[repr(C)]
    struct UnalignedStruct {
        a: u8,   // 1B
        b: u32,  // 4B
    }
    let mut buf = [0u8; 8];
    let p = unsafe { buf.as_ptr().add(1) as *const UnalignedStruct };
    // undefined behavior
    let _ = unsafe { core::ptr::read_volatile(p) };
}

fn test_true_Init() {
    let x = 77u32;
    let p: *const u32 = &x;
    let y = unsafe { core::ptr::read_volatile(p) };
    assert_eq!(y, 77u32);
}

fn test_false_Init_maybeuninit() {
    use std::mem::MaybeUninit;
    let x: MaybeUninit<u32> = MaybeUninit::uninit();
    let p = x.as_ptr();
    // undefined behavior
    let _ = unsafe { core::ptr::read_volatile(p) };
}

fn test_false_Init_array_partial() {
    use std::mem::MaybeUninit;
    let mut arr: [MaybeUninit<u32>; 3] = unsafe { MaybeUninit::uninit().assume_init() };
    // initialize the first two elements
    arr[0].write(1);
    arr[1].write(2);
    // arr[2] is still uninitialized
    let ptr = arr[2].as_ptr();
    let _ = unsafe { core::ptr::read_volatile(ptr) }; // UB - array element 'arr[2]' is not initialized
}

fn test_false_Init_struct_partial() {
    use std::mem::MaybeUninit;
    #[repr(C)]
    struct MyStruct {
        a: u32,
        b: u32,
    }
    
    let mut s: MaybeUninit<MyStruct> = MaybeUninit::uninit();
    
    // initialize the first field
    unsafe {
        s.as_mut_ptr().cast::<u32>().write(42);
    }
    
    // read the second field (uninitialized)
    let b_ptr = unsafe { s.as_ptr().cast::<u32>().add(1) };
    let _ = unsafe { core::ptr::read_volatile(b_ptr) }; // UB - struct field 'b' is not initialized
}

fn test_true_CopyTrait() {
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct MyCopy(u8);
    let x = MyCopy(1);
    let p: *const MyCopy = &x;
    let y = unsafe { core::ptr::read_volatile(p) };
    assert_eq!(y, x);
}

fn test_false_CopyTrait() {
    struct NotCopy(String);
    let x = NotCopy("hello".to_string());
    let p: *const NotCopy = &x;
    // undefined behavior
    let _ = unsafe { core::ptr::read_volatile(p) };
}

fn main() {
    //test_true_ValidPtr();
    // test_false_ValidPtr_null();
    // test_false_ValidPtr_use_after_free();
    //test_true_Aligned();
    // test_false_Aligned_byte_offset();
    // test_false_Aligned_dynamic_alloc();
    // test_false_Aligned_struct_field();
    // test_true_Init();
    // test_false_Init_maybeuninit();
    // test_false_Init_array_partial();
    // test_false_Init_struct_partial();
    // test_true_CopyTrait();
    // test_false_CopyTrait();
}
