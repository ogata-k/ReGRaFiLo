#[macro_use]
extern crate log;

use regrafilo::util::log::Logger;

fn main() {
    Logger::init(true, true);

    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warning");
    error!("error");
}
