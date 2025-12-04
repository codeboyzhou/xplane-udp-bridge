mod dataref;
mod udp;

use crate::dataref::DataRefReader;
use crate::udp::UdpClient;
use std::time::Duration;

fn main() {
    // Create UDP client
    let client = UdpClient::new("127.0.0.1", 49000, 30).expect("Failed to create UDP client");

    // Create DataRefReader
    let dataref_reader = DataRefReader::new(&client);

    loop {
        // Read dataref value examples
        let data_refs = ["sim/cockpit2/controls/parking_brake_ratio"];
        for data_ref in data_refs {
            match dataref_reader.read_as_float(data_ref) {
                Ok(value) => println!("✅  Dataref {data_ref} successfully read as float: {value}"),
                Err(err_msg) => eprintln!("❌ Error reading dataref: {}", err_msg),
            }
        }

        // Sleep for a short duration to avoid overloading the server
        std::thread::sleep(Duration::from_secs(3));
    }
}
