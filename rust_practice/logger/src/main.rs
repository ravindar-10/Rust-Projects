use log::{debug, warn,error,info,trace, LevelFilter};
use simplelog::*;
fn main() {
    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout, ColorChoice::Auto).unwrap();
    debug!("something interesting : {}",55);
    warn!("This is a warning");
    error!("something bad");
    info!("the data is not available");
    trace!("the account is not tradable");
    println!("Hello, world!");
}
