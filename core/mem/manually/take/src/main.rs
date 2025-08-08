use std::mem::ManuallyDrop;
#[derive(Debug)]
struct NoCopy {
    x: i32,
    y: Vec<i32>,
}

fn case_take1(val: i32) {
    let mut md = unsafe {
        ManuallyDrop::new(NoCopy {
            x: 1,
            y: vec![1, 2, 3],
        })
    };
    let mut value = unsafe { ManuallyDrop::take(&mut md) };
    if val == 0 {
        md.y[0] = val; // Use ManuallyDrop, UB
    }
    value.y[0] = val + 2;
    println!("value: {:?}", value);
    // unsafe { ManuallyDrop::drop(&mut md) }; UB: Double frop
}

fn main() {
    case_take1(0);
}
