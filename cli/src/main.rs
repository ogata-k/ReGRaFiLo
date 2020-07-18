use log::*;

use regrafilo_cli::logger::Logger;

fn main() {
    Logger::init(true);
    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warning");
    error!("error");

    Logger::init(true);
}
