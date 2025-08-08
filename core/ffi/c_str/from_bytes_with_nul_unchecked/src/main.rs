use std::ffi::CStr;

fn case_from_bytes_with_nul_unchecked1(len: u32) {
    if len == 0{
        let bytes: &[u8] = &[];
        let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(bytes) };
        println!("bytes: {:?}", bytes);
        println!("cstr: {:?}", cstr);
    }
    else{
        let bytes: &[u8] = &[1];
        let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(bytes) };
        println!("bytes: {:?}", bytes);
        println!("cstr: {:?}", cstr);
    }
}

fn main() {
    let len = 1;
    case_from_bytes_with_nul_unchecked1(len);
}
