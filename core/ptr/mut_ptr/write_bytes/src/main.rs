fn case_write_bytes1(path: i32) -> bool {
    //Typed
    if path > 0 {
        let mut buf = [0u8; 10];
        let ptr = buf.as_mut_ptr() as *mut u8;
        unsafe {
            ptr.write_bytes(0x55, 10);
        }
        println!("buf: {:?}", buf);
    } else {
        unsafe {
            let mut value: u8 = 0;
            let ptr: *mut bool = &mut value as *mut u8 as *mut bool;
            let _bool = ptr.read(); // This is fine, `ptr` points to a valid `bool`.
            ptr.write_bytes(42u8, 1); // This function itself does not cause UB...
            let _bool = ptr.read(); // ...but it makes this operation UB! ⚠️
        }
    }
    true
}

fn case_write_bytes2(path: u32) -> bool {
    //ValidPtr
    match path {
        0 => {
            // Normal case - valid pointer
            let mut buf = [0u8; 10];
            let ptr = buf.as_mut_ptr() as *mut u8;
            unsafe {
                ptr.write_bytes(0x55, 5);
            }
            println!("buf: {:?}", buf);
        }
        1 => {
            // ValidPtr violation - null pointer
            let ptr: *mut u8 = std::ptr::null_mut();
            unsafe {
                ptr.write_bytes(0x55, 1); // UB: null pointer
            }
        }
        2 => {
            // ValidPtr violation - dangling pointer (use after free)
            let layout = std::alloc::Layout::new::<u8>();
            let ptr = unsafe { std::alloc::alloc(layout) as *mut u8 };
            if ptr.is_null() {
                return false;
            }
            unsafe {
                std::alloc::dealloc(ptr, layout); // Deallocate first
                ptr.write_bytes(0x55, 1); // UB: use after free
            }
        }
        3 => {
            // ValidPtr violation - out of bounds access
            let mut buf = [0u8; 5];
            let ptr = &mut buf[0] as *mut u8;
            unsafe {
                ptr.write_bytes(0x55, 10); // UB: writing beyond buffer bounds
            }
        }
        _ => {
            // Default: normal case
            let mut buf = [0u8; 10];
            let ptr = &mut buf[0] as *mut u8;
            unsafe {
                ptr.write_bytes(0x55, 3);
            }
            println!("Default case: buf[0..3] = {:?}", &buf[0..3]);
        }
    }
    true
}

fn case_write_bytes3(path: u32) -> bool {
    //Aligned
    match path {
        0 => {
            // Normal case - properly aligned pointer
            let mut buf = [0u8; 16];
            let ptr = buf.as_mut_ptr() as *mut u8;
            unsafe {
                ptr.write_bytes(0x55, 8);
            }
            println!("buf: {:?}", buf);
        }
        1 => {
            // Aligned violation - byte offset causing misalignment
            let mut buf = [0u8; 16];
            let ptr = &mut buf[0] as *mut u8; // Offset by 1 byte
            let ptr = unsafe { ptr.byte_add(1) };
            unsafe {
                // Cast to u32 pointer which requires 4-byte alignment
                let u32_ptr = ptr as *mut u32;
                u32_ptr.write_bytes(0x1u8, 1); // UB: misaligned u32 pointer
            }
            println!("buf: {:?}", buf)
        }
        2 => {
            // Aligned violation - struct field misalignment
            #[derive(Debug)]
            #[derive(Copy, Clone)]
            #[repr(C)]
            struct UnalignedStruct {
                a: u8,  // 1B
                b: u32, // 4B
            }
            let mut buf = [0u8; 9];
            let p = unsafe { buf.as_mut_ptr().add(1) as *mut UnalignedStruct };
            // undefined behavior
            let _ = unsafe { p.write_bytes(0x1u8, 1) };
            unsafe {
                println!("*p: {:?}", *p);
            }
            // #[repr(packed)]
            // #[derive(Debug)]
            // struct PackedStruct {
            //     a: u16,
            //     b: bool,
            //     c: u16, // Requires 4-byte alignment but packed struct may not provide it
            // }

            // let mut packed = PackedStruct { a: 0, b: false, c: 0 };
            // let ptr = std::ptr::addr_of_mut!(packed.c) as *mut u;
            // unsafe {
            //     let u16_ptr = ptr as *mut u16;
            //     u16_ptr.write_bytes(0x1u8, 1); // UB: potentially misaligned
            // }
            // println!("packed: {:?}", packed);
            // unsafe {
            //     println!("packed.c: {:?}", *ptr);
            // }
        }
        3 => {
            // Aligned violation - dynamic allocation with wrong alignment
            let layout = std::alloc::Layout::from_size_align(8, 4).unwrap(); // 1-byte alignment for u32
            let ptr = unsafe { std::alloc::alloc(layout) as *mut u8 };
            if ptr.is_null() {
                return false;
            }
            unsafe {
                let p = ptr.add(1) as *mut u32;
                p.write_bytes(0x1u8, 1); // UB: u32 pointer with 1-byte alignment
                println!("*p: {:?}", *p);
                std::alloc::dealloc(ptr, layout);
            }
        }
        _ => {
            // Default: normal case
            let mut buf = [0u8; 16];
            let ptr = &mut buf[0] as *mut u8;
            unsafe {
                ptr.write_bytes(0x55, 4);
            }
            println!("Default case: buf[0..4] = {:?}", &buf[0..4]);
        }
    }
    true
}

fn main() {
    case_write_bytes1(1);
    case_write_bytes2(0);
    case_write_bytes3(3);
}
