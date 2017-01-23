#[macro_use]
extern crate log;
extern crate env_logger;

use log::LogLevel;

fn main() {
    env_logger::init().unwrap();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(LogLevel::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
