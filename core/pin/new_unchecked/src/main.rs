use std::mem;
use std::pin::Pin;
use std::fmt::Debug;
use std::rc::Rc;

fn case_move_pinned_ref<T: Debug>(mut a: T, mut b: T) {
    unsafe {
        let p: Pin<&mut T> = Pin::new_unchecked(&mut a);
        // This should mean the pointee `a` can never move again.
    }
    mem::swap(&mut a, &mut b); // Potential UB down the road ⚠️
    // The address of `a` changed to `b`'s stack slot, so `a` got moved even
    // though we have previously pinned it! We have violated the pinning API contract.
    println!("a: {:?}, b: {:?}", a, b); // Pinned Violated
}

fn case_move_pinned_rc<T: Debug + Default>(mut x: Rc<T>) {
    // This should mean the pointee can never move again.
    let pin = unsafe { Pin::new_unchecked(Rc::clone(&x)) };
    {
        let p: Pin<&T> = pin.as_ref();
        // ...
    }
    drop(pin);

    let content = Rc::get_mut(&mut x).unwrap(); // Potential UB down the road ⚠️
    // Now, if `x` was the only reference, we have a mutable reference to
    // data that we pinned above, which we could use to move it as we have
    // seen in the previous example. We have violated the pinning API contract.
    let t = std::mem::take(content); // Pinned Violated
    println!("x: {:?}", x);
    println!("t: {:?}", t);
}

fn main() {
    let a = String::from("Hello");
    let b = String::from("World");
    case_move_pinned_ref(a, b);

    let x = Rc::new(String::from("Hello"));
    case_move_pinned_rc(x);
}