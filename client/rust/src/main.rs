mod dataref;
mod udp;

use crate::dataref::DataRefReader;
use crate::udp::UdpClient;
use nu_ansi_term::Color::{Green, Red};
use std::time::Duration;

fn main() {
    // Create UDP client
    let client = UdpClient::new("127.0.0.1", 49000, 3).expect("Failed to create UDP client");

    // Create DataRefReader
    let dataref_reader = DataRefReader::new(&client);

    loop {
        // Read dataref value examples
        let data_refs = ["sim/cockpit2/controls/parking_brake_ratio"];
        for data_ref in data_refs {
            match dataref_reader.read_as_float(data_ref) {
                Ok(value) => println!(
                    "{}",
                    Green.paint(format!(
                        "Dataref {} successfully read as float: {}",
                        data_ref, value
                    ))
                ),
                Err(msg) => {
                    eprintln!("{}", Red.paint(format!("Dataref {} read failed: {}", data_ref, msg)))
                }
            }
        }

        // Sleep for a short duration to avoid overloading the server
        std::thread::sleep(Duration::from_secs(3));
    }
}
