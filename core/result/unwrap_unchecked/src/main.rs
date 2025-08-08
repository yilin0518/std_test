fn case_unwrap_unchecked1(val: i32) -> bool {
    let x1 :Result<u32, &str> = Ok(1);
    let x2 :Result<u32, &str> = Err("error");
    if val == 0{
        unsafe {
            let _ = x1.unwrap_unchecked();
        }
    }else{
        unsafe {
            let _ = x2.unwrap_unchecked();
        }
    }
    true
}

fn main() {
    case_unwrap_unchecked1(1);
}
