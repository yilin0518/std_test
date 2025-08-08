#![feature(layout_for_ptr, ptr_metadata)]
use std::alloc::Layout;
use std::mem;
use std::ptr::{self, NonNull};

// Trait for testing trait object cases
trait DummyTrait {
    fn dummy(&self) {}
}

// Struct implementing the trait
struct DummyStruct;
impl DummyTrait for DummyStruct {
    fn dummy(&self) {}
}

// Struct for a different vtable
struct OtherStruct;
impl DummyTrait for OtherStruct {
    fn dummy(&self) {}
}

// Unsound function 1: Slice with uninitialized length (flag < 0) or valid slice (flag >= 0)
pub fn unsound_slice_uninitialized<T>(flag: i32) -> Layout {
    if flag >= 0 {
        // Normal path: Create a valid slice with initialized length
        let data = [0u8; 4]; // Small, valid array
        let slice_ptr = data.as_slice() as *const [u8];
        // SAFETY: The slice pointer is valid, points to initialized data,
        // and its size (4 bytes) fits within `isize::MAX`.
        unsafe { Layout::for_value_raw(slice_ptr as *const [T]) }
    } else {
        // Unsound path: Create a slice with uninitialized length field
        let uninit: mem::MaybeUninit<(usize, *const T)> = mem::MaybeUninit::uninit();
        let slice_ptr = ptr::slice_from_raw_parts(uninit.as_ptr() as *const T, uninit.as_ptr() as usize);
        // SAFETY VIOLATION: The slice's length is uninitialized, as it comes from
        // uninit.as_ptr() cast to usize. This may cause `size_of_val_raw` to read
        // undefined data, leading to UB (e.g., arbitrary length values).
        unsafe { Layout::for_value_raw(slice_ptr) }
    }
}

// Unsound function 2: Slice with oversized length (flag < 0) or valid slice (flag >= 0)
pub fn unsound_slice_oversized<T>(flag: i32) -> Layout {
    if flag >= 0 {
        // Normal path: Create a valid slice with reasonable length
        let data = vec![0u8; 10]; // Small, valid vector
        let slice_ptr = data.as_slice() as *const [u8];
        // SAFETY: The slice pointer is valid, points to initialized data,
        // and its size (10 bytes) fits within `isize::MAX`.
        unsafe { Layout::for_value_raw(slice_ptr as *const [T]) }
    } else {
        // Unsound path: Create a slice with an extremely large length
        let large_len = usize::MAX;
        let slice_ptr = ptr::slice_from_raw_parts::<T>(ptr::null(), large_len);
        // SAFETY VIOLATION: The slice's total size (len * size_of::<T>()) may exceed
        // `isize::MAX`, violating the safety requirement of `for_value_raw`. This can
        // cause `size_of_val_raw` to return an incorrect size or trigger UB due to
        // arithmetic overflow or invalid memory access.
        unsafe { Layout::for_value_raw(slice_ptr) }
    }
}

// Unsound function 3: Trait object with invalid vtable (flag < 0) or valid trait object (flag >= 0)
pub fn unsound_trait_object_invalid_vtable(flag: i32) -> Layout {
    if flag >= 0 {
        // Normal path: Create a valid trait object
        let dummy = DummyStruct;
        let trait_obj: &dyn DummyTrait = &dummy;
        let trait_obj_ptr = trait_obj as *const dyn DummyTrait;
        // SAFETY: The pointer is a valid trait object created via unsizing coercion,
        // and its size fits within `isize::MAX`.
        unsafe { Layout::for_value_raw(trait_obj_ptr) }
    } else {
        // Unsound path: Create a trait object with a fake vtable
        let dummy = DummyStruct;
        let data_ptr = &dummy as *const DummyStruct;
        // Use a dangling pointer as a fake vtable, transmuted to DynMetadata
        let fake_vtable = unsafe {
            mem::transmute::<*const (), ptr::DynMetadata<dyn DummyTrait>>(0xDEADBEEF as *const ())
        };
        let trait_obj_ptr = ptr::from_raw_parts::<dyn DummyTrait>(data_ptr as *const (), fake_vtable);
        // SAFETY VIOLATION: The vtable pointer is invalid (points to 0xDEADBEEF, not a
        // valid vtable created via unsizing coercion). Calling `for_value_raw` may
        // attempt to dereference the invalid vtable, leading to UB (e.g., segmentation
        // fault or incorrect layout calculation).
        unsafe { Layout::for_value_raw(trait_obj_ptr) }
    }
}

// Unsound function 4: Trait object with mismatched vtable (flag < 0) or valid trait object (flag >= 0)
pub fn unsound_trait_object_mismatched_vtable(flag: i32) -> Layout {
    if flag >= 0 {
        // Normal path: Create a valid trait object
        let dummy = DummyStruct;
        let trait_obj: &dyn DummyTrait = &dummy;
        let trait_obj_ptr = trait_obj as *const dyn DummyTrait;
        // SAFETY: The pointer is a valid trait object created via unsizing coercion,
        // and its size fits within `isize::MAX`.
        unsafe { Layout::for_value_raw(trait_obj_ptr) }
    } else {
        // Unsound path: Create a trait object with mismatched vtable
        let dummy = DummyStruct;
        let data_ptr = ptr::addr_of!(dummy) as *const ();
        // Create another type with a different vtable
        let other = OtherStruct;
        let other_trait_obj: &dyn DummyTrait = &other;
        let other_vtable = ptr::metadata(other_trait_obj);
        let mismatched_trait_obj = ptr::from_raw_parts::<dyn DummyTrait>(data_ptr, other_vtable);
        // SAFETY VIOLATION: The vtable is for `OtherStruct`, but the data pointer points
        // to `DummyStruct`. This mismatch may cause `size_of_val_raw` or `align_of_val_raw`
        // to access incorrect metadata, leading to UB or incorrect layout.
        unsafe { Layout::for_value_raw(mismatched_trait_obj) }
    }
}

fn main() {
    // Test normal paths
    println!("Testing normal paths (flag = 0):");
    let layout1 = unsound_slice_uninitialized::<u8>(-1);
    println!("unsound_slice_uninitialized: {:?}", layout1);
    let layout2 = unsound_slice_oversized::<u8>(0);
    println!("unsound_slice_oversized: {:?}", layout2);
    let layout3 = unsound_trait_object_invalid_vtable(0);
    println!("unsound_trait_object_invalid_vtable: {:?}", layout3);
    let layout4 = unsound_trait_object_mismatched_vtable(0);
    println!("unsound_trait_object_mismatched_vtable: {:?}", layout4);

    // Test unsound paths (MAY CAUSE UB)
    println!("\nTesting unsound paths (flag = -1):");
    let layout1 = unsound_slice_uninitialized::<u8>(-1);
    println!("unsound_slice_uninitialized: {:?}", layout1);
    // let layout2 = unsound_slice_oversized::<u8>(-1);
    // println!("unsound_slice_oversized: {:?}", layout2);
    // let layout3 = unsound_trait_object_invalid_vtable(-1);
    // println!("unsound_trait_object_invalid_vtable: {:?}", layout3);
    let layout4 = unsound_trait_object_mismatched_vtable(-1);
    println!("unsound_trait_object_mismatched_vtable: {:?}", layout4);
}