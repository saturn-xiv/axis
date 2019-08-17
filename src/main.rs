#[macro_use]
extern crate log;
extern crate axis;

fn main() {
    if let Err(err) = axis::app::launch() {
        error!("{:?}", err);
    }
}
