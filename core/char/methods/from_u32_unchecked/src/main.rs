fn case_from_u32_unchecked1(val: u32) {
    let c = unsafe { char::from_u32_unchecked(val) };
    println!("c: {}", c);
}

fn main() {
    case_from_u32_unchecked1(0x88888888);
}
