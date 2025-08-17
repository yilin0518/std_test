#![allow(invalid_null_arguments)]

// ValidPtr
fn case_replace1(val: i32) -> bool {
    match val {
        1 => {
            let mut x = 42i32;
            let p: *mut i32 = &mut x as *mut i32;
            let y = unsafe { core::ptr::replace(p, 10) };
            assert_eq!(y, 42);
            println!("x: {}", x);
        }
        2 => {
            // ValidPtr violation - invalid pointer
            let x = 42i32;
            let p: *mut i32 = &x as *const i32 as *mut i32;
            let y = unsafe { core::ptr::replace(p, 10) };
            assert_eq!(y, 42);
        }
        3 => {
            // Null
            let p: *mut i32 = std::ptr::null_mut();
            // undefined behavior
            let _ = unsafe { core::ptr::replace(p, 10) };
        }
        4 => {
            // Use after free
            let p: *mut i32;
            {
                let mut x = 123i32;
                p = &mut x as *mut i32;
            }
            let _ = unsafe { core::ptr::replace(p, 10) };
        }
        _ => {
            let mut x = 42i32;
            let p: *mut i32 = &mut x as *mut i32;
            let y = unsafe { core::ptr::replace(p, 10) };
            assert_eq!(y, 42);
            println!("x: {}", x);
        }
    };
    true
}

// Aligned
fn case_replace2(val: i32) -> bool {
    match val {
        0 => {
            let mut x = 42;
            let p: *mut i32 = &mut x as *mut i32;
            let y = unsafe { core::ptr::replace(p, 10) };
            assert_eq!(y, 42);
            println!("x: {}", x);
        }
        1 => {
            let mut buf = [0u8; 8];
            // unaligned pointer
            let p = unsafe { buf.as_ptr().add(1) as *const u32 as *mut u32 };
            // undefined behavior
            let _ = unsafe { core::ptr::replace(p, 10) };
        }
        2 => {
            let layout = std::alloc::Layout::from_size_align(8, 4).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };
            // initialize the allocated memory
            unsafe {
                std::ptr::write_bytes(ptr, 0, 8);
            }
            let p = unsafe { ptr.add(1) as *const u32 as *mut u32 };
            // undefined behavior
            let _ = unsafe { core::ptr::replace(p, 10) };
            unsafe {
                std::alloc::dealloc(ptr, layout);
            }
        }
        3 => {
            #[repr(C)]
            struct UnalignedStruct {
                a: u8,  // 1B
                b: u32, // 4B
            }
            let mut buf = [0u8; 9];
            let p =
                unsafe { buf.as_ptr().add(1) as *const UnalignedStruct as *mut UnalignedStruct };
            // undefined behavior
            let _ = unsafe { core::ptr::replace(p, UnalignedStruct { a: 1, b: 2 }) };
        }
        _ => {}
    };
    true
}

// Init
fn case_replace3(val: i32) -> bool {
    match val {
        0 => {
            let mut x = 42u32;
            let p: *mut u32 = &mut x as *mut u32;
            let y = unsafe { core::ptr::replace(p, 10) };
            assert_eq!(y, 42);
            println!("x: {}", x);
        }
        1 => {
            use std::mem::MaybeUninit;
            let mut x: MaybeUninit<u64> = MaybeUninit::uninit();
            let p = x.as_mut_ptr();
            // undefined behavior
            let _ = unsafe { core::ptr::replace(p, 10) };
        }
        2 => {
            use std::mem::MaybeUninit;
            let mut arr: [MaybeUninit<u32>; 3] = unsafe { MaybeUninit::uninit().assume_init() };
            // initialize the first two elements
            arr[0].write(1);
            arr[1].write(2);
            // arr[2] is still uninitialized
            let ptr = arr[2].as_mut_ptr();
            let _ = unsafe { core::ptr::replace(ptr, 10) }; // UB - array element 'arr[2]' is not initialized
        }
        3 => {
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
            let b_ptr = unsafe { s.as_mut_ptr().cast::<u32>().add(1) };
            let _ = unsafe { core::ptr::replace(b_ptr, 10) }; // UB - struct field 'b' is not initialized
        }
        _ => {}
    };
    true
}

fn main() {
    //case_replace1(1);
    //case_replace2(3);
    case_replace3(0);
}
