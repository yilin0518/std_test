# Safety Properties (SP)  Case Patterns

This document summarizes various  false case patterns for each Primitive Safety Property (SP) in unsafe APIs.

## 1. ValidPtr SP

**Meaning**: Pointer must point to valid, allocated memory

### False Case Patterns:

#### 1.1 Null Pointer

```rust
let p: *const u32 = std::ptr::null();
let _ = unsafe { core::ptr::read(p) }; // UB
```

#### 1.2 Dangling Pointer / Use-after-free

```rust
let p: *const u32;
{
    let x = 123u32;
    p = &x;
} // x's lifetime ends, p becomes dangling pointer
let _ = unsafe { core::ptr::read(p) }; // UB
```

## 2. Null SP

**Meaning**: Pointer must be null pointer

### True Case Patterns:

#### 2.1 Null Pointer

```rust
let p: *mut u32 = std::ptr::null_mut();
let r = unsafe { p.as_ref() };
assert_eq!(r, None);
```

## 3. ValidPtr2Ref SP

**Meaning**: Validity of pointer to reference conversion, mathematically represented as `Ptr2Ref(p, T) = Init(p, T, 1) && Align(p, T) && Alias(p, 0)`

### False Case Patterns:

#### 3.1 Violating Init(p, T, 1) - Memory not initialized

```rust
use std::mem::MaybeUninit;
let mut x: MaybeUninit<u32> = MaybeUninit::uninit();
let p = x.as_mut_ptr();
let r = unsafe { p.as_ref_unchecked() }; // UB - memory not initialized
```

#### 3.2 Violating Align(p, T) - Pointer not aligned

```rust
let mut buf = [0u8; 8];
let p = unsafe { buf.as_ptr().add(1) as *mut u32 };
let r = unsafe { p.as_ref_unchecked() }; // UB - pointer not aligned
```

#### 3.3 Violating Alias(p, 0) - Creating aliases

```rust
let mut x = 42u32;
let p: *mut u32 = &mut x as *mut u32;
let r1 = unsafe { p.as_ref_unchecked() };
unsafe { *p = 43; } // Modify through pointer, but r1 still points to original value
println!("{:?}", r1); // Violates exclusive mutability principle
```

## 4. InBounded SP

**Meaning**: `mem(p, p+ sizeof(T) * len)` belongs to single allocated object

### False Case Patterns:

#### 4.1 Out of bounds access

```rust
let arr = [1u32, 2, 3];
let p: *const u32 = arr.as_ptr();
let p2 = unsafe { p.add(5) }; // Out of array bounds
let _ = unsafe { *p2 }; // UB
```

## 5. ValidNum SP

**Meaning**: Each value in the array belongs to valid range

### False Case Patterns:

#### 5.1 Values out of valid range

```rust
let arr = [u32::MAX, 2, 3]; // Contains values InBound
let p: *const u32 = arr.as_ptr();
let p2 = unsafe { p.add(0) };
let _ = unsafe { *p2 + 1 }; // Reading value out of range
```

## 6. Aligned SP

**Meaning**: Pointer must satisfy type alignment requirements

### False Case Patterns:

#### 6.1 Byte offset misaligned

```rust
let mut buf = [0u8; 8];
let p = unsafe { buf.as_ptr().add(1) as *const u32 };
let _ = unsafe { core::ptr::read(p) }; // UB
```

#### 6.2 Dynamic allocation misaligned

```rust
let layout = std::alloc::Layout::from_size_align(8, 4).unwrap();
let ptr = unsafe { std::alloc::alloc(layout) };
unsafe { std::ptr::write_bytes(ptr, 0, 8); }
let p = unsafe { ptr.add(1) as *const u32 };
let _ = unsafe { core::ptr::read(p) }; // UB
unsafe { std::alloc::dealloc(ptr, layout); }
```

#### 6.3 Struct field misaligned

```rust
#[repr(C)]
struct UnalignedStruct {
    a: u8,   // 1B
    b: u32,  // 4B
}
let mut buf = [0u8; 8];
let p = unsafe { buf.as_ptr().add(1) as *const UnalignedStruct };
let _ = unsafe { core::ptr::read(p) }; // UB
```

## 7. Init SP

**Meaning**: Memory pointed to must be initialized to valid T type value

### False Case Patterns:

#### 7.1 MaybeUninit uninitialized

```rust
use std::mem::MaybeUninit;
let x: MaybeUninit<u64> = MaybeUninit::uninit();
let p = x.as_ptr();
let _ = unsafe { core::ptr::read(p) }; // UB
```

#### 7.2 Array partially initialized

```rust
let arr: [MaybeUninit<u32>; 3] = unsafe { MaybeUninit::uninit().assume_init() };
arr[0].write(1);
arr[1].write(2);
// arr[2] is still uninitialized
let ptr = arr[2].as_ptr();
let _ = unsafe { core::ptr::read(ptr) }; // UB
```

#### 7.3 Struct partially initialized

```rust
#[repr(C)]
struct MyStruct {
    a: u32,
    b: u32,
}

let mut s: MaybeUninit<MyStruct> = MaybeUninit::uninit();
unsafe {
    s.as_mut_ptr().cast::<u32>().write(42);
}
let b_ptr = unsafe { s.as_ptr().cast::<u32>().add(1) };
let _ = unsafe { core::ptr::read(b_ptr) }; // UB
```

## 8. CopyTrait SP

**Meaning**: Type T must implement Copy trait

### False Case Patterns:

#### 8.1 Non-Copy type

```rust
struct NotCopy(String);
let x = NotCopy("hello".to_string());
let p: *const NotCopy = &x;
let _ = unsafe { core::ptr::read(p) }; // UB - may cause double drop
```
