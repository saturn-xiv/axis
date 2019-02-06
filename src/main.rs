#[macro_use]
extern crate log;
extern crate axis;

fn main() {
    if let Err(err) = axis::launch() {
        error!("{:?}", err);
    }
}
