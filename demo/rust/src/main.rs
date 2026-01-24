mod dataref;
mod udp;

use crate::dataref::DataRefReader;
use crate::udp::UdpClient;
use nu_ansi_term::Color::{Green, Red};
use std::collections::HashMap;
use std::time::Duration;

fn main() {
    // Create UDP client
    let client = UdpClient::new("127.0.0.1", 49000, 3).expect("Failed to create UDP client");

    // Create DataRefReader
    let dataref_reader = DataRefReader::new(&client);

    // Read dataref value examples
    let data_refs = HashMap::from([
        ("sim/cockpit2/controls/parking_brake_ratio", "float"),
        ("sim/cockpit2/engine/actuators/throttle_ratio", "float"),
        ("sim/cockpit2/engine/actuators/eng_master", "[int]"),
        ("sim/cockpit2/electrical/battery_on", "[int]"),
    ]);

    loop {
        for (data_ref, type_str) in &data_refs {
            let value = match *type_str {
                "int" => dataref_reader.read(data_ref, type_str),
                "float" => dataref_reader.read(data_ref, type_str),
                "[int]" => dataref_reader.read(data_ref, type_str),
                "[float]" => dataref_reader.read(data_ref, type_str),
                _ => Err(format!("Unsupported type {} for dataref {}", type_str, data_ref)),
            };

            match value {
                Ok(value) => println!(
                    "{}",
                    Green.paint(format!(
                        "Dataref {} successfully read as {}: {}",
                        data_ref, type_str, value
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
