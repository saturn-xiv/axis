#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    if let Err(e) = axis::app::run() {
        error!("{:?}", e);
    }
}
