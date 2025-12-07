mod dataref;
mod error;

use crate::dataref::MockDataRefReader;
use infra::{logger, udp};

fn main() {
    logger::init_file_logger("mock.log");
    udp::server::start(49000);
    udp::server::register_request_handler(Box::new(MockDataRefReader::new()));
    println!("Mock server started and listening on port 49000");
    // keep the mock server running
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
