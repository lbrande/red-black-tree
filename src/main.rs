use std::boxed::Box;

fn main() {
    let mut val = Box::new(5);
    let raw = val.as_mut() as *mut i32;
    let box_raw = Box::into_raw(val);
    dbg!(raw);
    dbg!(box_raw);
}