#[macro_use]
extern crate log;

extern crate axis;
extern crate env_logger;

fn main() {
    env_logger::init();
    if let Err(e) = axis::app::run() {
        error!("{:?}", e);
    }
}
