// FIXME: Make me compile! Diff budget: 1 line.

pub fn main() {
    let x = 10;
    unsafe { *(x as *mut i32) = 20; }
}
