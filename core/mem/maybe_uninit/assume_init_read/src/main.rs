use std::mem::MaybeUninit;
fn case_assume_init_read1(input: i32) -> bool {
    if input < 0 {
        unsafe {
            let mut x = MaybeUninit::<i32>::uninit();
            x.write(input);
            let val = x.assume_init_read();
            true
        }
    } else {
        unsafe {
            let mut x = MaybeUninit::<i32>::uninit();
            let val = x.assume_init_read(); // UnInit
            true
        }
    }
}

#[derive(Debug, PartialEq)]
struct NoCopy{
    x: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ImplCopy{
    x: i32,
}

fn case_assume_init_read2(input: i32) -> bool {
    if input < 0 {
        unsafe {
            let mut x = MaybeUninit::<ImplCopy>::uninit();
            x.write(ImplCopy{x: input});
            let x1 = x.assume_init_read();
            let x2 = x.assume_init_read();
            assert_eq!(x1, x2);
        }
        true
    } else {
        unsafe {
            let mut x = MaybeUninit::<Option<Vec<i32>>>::uninit();
            x.write(Some(vec![input]));
            let x1 = x.assume_init_read(); // Not Impl Copy
            let x2 = x.assume_init_read();
            assert_eq!(x1, x2);
        }
        true
    }
}

fn main() {
    let res = case_assume_init_read2(-1);
    println!("res: {}", res);
    let res = case_assume_init_read2(1);
    println!("res: {}", res);
}
