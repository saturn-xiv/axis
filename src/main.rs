extern crate axis;

fn main() {
    if let Err(err) = axis::launch() {
        panic!(err)
    }
}
