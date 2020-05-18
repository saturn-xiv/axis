#[macro_use]
extern crate log;

#[actix_rt::main]
async fn main() {
    env_logger::init();
    if let Err(e) = axis::app::run().await {
        error!("{}", e);
    }
}
