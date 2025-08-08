use std::mem;

#[derive(Debug)]
struct Foo {
    a: bool, // 1byte
    b: *const i8, // 8bytes
    d: i16, // 2bytes
    c: i8, // 1byte
    e: i32, // 4bytes
} //  bytes

fn main() {
    let foo_array = [1u16; 8];
    let foo_struct: Foo = unsafe { mem::transmute_copy(&foo_array) };
    println!("Foo_size: {:?}", std::mem::size_of::<Foo>());
    println!("Foo_align: {:?}", std::mem::align_of::<Foo>());
    println!("foo_struct: {:?}", foo_struct);
}
