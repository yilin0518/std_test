#[allow(invalid_null_arguments)]
fn case_read1(val: i32) -> bool {
    // ValidPtr
    if val > 1 {
        let x = val;
        let p: *const i32 = &x;
        let y = unsafe { core::ptr::read(p) };
        assert_eq!(y, val);
    } else if val == 1 {
        // Pointer to a box, which will cause double free
        let x = Box::new(42i32);
        let x_ptr: *mut Box<i32> = &x as *const Box<i32> as *mut Box<i32>;
        unsafe {
            let y = core::ptr::read(x_ptr);
            println!("y: {}", *y);
            println!("x: {}", *x);
        }
    } else if val == 0 {
        // Null
        let p: *const i32 = std::ptr::null();
        // undefined behavior
        let _ = unsafe { core::ptr::read(p) };
    } else {
        // Use after free
        let p: *const i32;
        {
            let x = 123i32;
            p = &x;
        }
        let _ = unsafe { core::ptr::read(p) };
    };
    true
}

fn case_read2(val: i32) -> bool {
    // Aligned
    match val {
        0 => {
            let x = val;
            let p: *const i32 = &x;
            let y = unsafe { core::ptr::read(p) };
            assert_eq!(y, val);
        }
        1 => {
            let mut buf = [0u8; 8];
            // unaligned pointer
            let p = unsafe { buf.as_ptr().add(1) as *const u32 };
            // undefined behavior
            let _ = unsafe { core::ptr::read(p) };
        }
        2 => {
            let layout = std::alloc::Layout::from_size_align(8, 4).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };
            // initialize the allocated memory
            unsafe {
                std::ptr::write_bytes(ptr, 0, 8);
            }
            let p = unsafe { ptr.add(1) as *const u32 };
            // undefined behavior
            let _ = unsafe { core::ptr::read(p) };
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
            let p = unsafe { buf.as_ptr().add(1) as *const UnalignedStruct };
            // undefined behavior
            let _ = unsafe { core::ptr::read(p) };
        }
        _ => {}
    };
    true
}

fn case_read3(val: i32) -> bool {
    match val {
        0 => {
            let x = 42u32;
            let p: *const u32 = &x;
            let y = unsafe { core::ptr::read(p) };
            assert_eq!(y, 42);
        }
        1 => {
            use std::mem::MaybeUninit;
            let x: MaybeUninit<u64> = MaybeUninit::uninit();
            let p = x.as_ptr();
            // undefined behavior
            let _ = unsafe { core::ptr::read(p) };
        }
        2 => {
            use std::mem::MaybeUninit;
            let mut arr: [MaybeUninit<u32>; 3] = unsafe { MaybeUninit::uninit().assume_init() };
            // initialize the first two elements
            arr[0].write(1);
            arr[1].write(2);
            // arr[2] is still uninitialized
            let ptr = arr[2].as_ptr();
            let _ = unsafe { core::ptr::read(ptr) }; // UB - array element 'arr[2]' is not initialized
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
            let b_ptr = unsafe { s.as_ptr().cast::<u32>().add(1) };
            let _ = unsafe { core::ptr::read(b_ptr) }; // UB - struct field 'b' is not initialized
        }
        _ => {}
    };
    true
}

fn case_read4(val: i32) -> bool {
    match val {
        0 => {
            #[derive(Copy, Clone, Debug, PartialEq)]
            struct MyCopy(u8);
            let x = MyCopy(1);
            let p: *const MyCopy = &x;
            let y = unsafe { core::ptr::read(p) };
            assert_eq!(y, x);
        }
        1 => {
            struct NotCopy(String);
            let x = NotCopy("hello".to_string());
            let p: *const NotCopy = &x;
            // undefined behavior
            let _ = unsafe { core::ptr::read(p) };
        }
        _ => {}
    };
    true
}

fn main() {
    let _ = case_read1(1);
    // let _ = case_read2(3);
    //let _ = case_read3(2);
    //let _ = case_read4(1);
}
