# XPlane UDP Bridge

A UDP bridge plugin for X-Plane that enables external applications to communicate with the simulator through UDP protocol. This project provides a robust infrastructure for reading data references (datarefs) from X-Plane and includes client implementations in multiple programming languages.

## Overview

XPlaneUDPBridge consists of two main components:

1. **X-Plane Plugin**: A native plugin that runs inside X-Plane and exposes a UDP server for external communication
2. **Client Libraries**: Implementations in Python, Go, and Rust that demonstrate how to communicate with the plugin

The plugin allows external applications to read X-Plane data references (datarefs) without requiring direct integration with X-Plane's SDK, making it easier to build external tools, dashboards, or applications that interact with flight data.

## Features

- **Multi-language Support**: Client libraries available in Python, Go, and Rust
- **High Performance**: Asynchronous UDP server implementation using Tokio
- **Type Safety**: Support for different data types (int, float, int arrays, float arrays)
- **Robust Error Handling**: Comprehensive error handling and logging
- **Easy Integration**: Simple protocol for reading X-Plane data references

## Project Structure

```
xplane-udp-bridge/
├── demo/                 # Client demo implementations
│   ├── go/                # Go client implementation
│   ├── python/            # Python client implementation
│   └── rust/              # Rust client implementation
└── plugin/                # X-Plane plugin
    ├── dylib/             # Main plugin implementation
    ├── infra/             # Shared infrastructure
    └── mock/              # Mock implementation for testing
```

## Installation

### Prerequisites

- X-Plane 11 or X-Plane 12
- Rust toolchain (for building the plugin)
- Appropriate development environment for your preferred client language

### Building the Plugin

1. Clone the repository:
   ```bash
   git clone https://github.com/codeboyzhou/xplane-udp-bridge.git
   cd xplane-udp-bridge
   ```

2. Set up environment variables (copy `.env.example` to `.env` and update):
   ```bash
   XPLANE_PLUGIN_DIR=<path-to-your-xplane-plugins-directory>
   LIBCLANG_PATH=<path-to-libclang-binary>
   ```

3. Build the plugin:
   ```bash
   cd plugin
   # This will build and copy the files to the X-Plane plugins directory automatically
   build.bat
   ```

4. Reload the plugin or Restart X-Plane

## Usage

### Python Client

```python
from udp import UdpClient
from dataref import DataRefReader

# Create UDP client
client = UdpClient("127.0.0.1", 49000)

# Create DataRefReader
dataref_reader = DataRefReader(client)

# Read dataref values
parking_brake = dataref_reader.read("sim/cockpit2/controls/parking_brake_ratio", "float")
throttle = dataref_reader.read("sim/cockpit2/engine/actuators/throttle_ratio", "float")

print(f"Parking brake: {parking_brake}")
print(f"Throttle: {throttle}")
```

### Go Client

```go
package main

import (
    "fmt"
    "time"
)

func main() {
    // Create UDP client
    client := NewUdpClient("127.0.0.1", 49000, 3)
    
    // Create DataRefReader
    reader := NewDataRefReader(client)
    
    // Read dataref value
    value := reader.Read("sim/cockpit2/controls/parking_brake_ratio", "float")
    fmt.Printf("Parking brake: %s\n", value)
    
    time.Sleep(3 * time.Second)
}
```

### Rust Client

```rust
use crate::dataref::DataRefReader;
use crate::udp::UdpClient;

fn main() {
    // Create UDP client
    let client = UdpClient::new("127.0.0.1", 49000, 3).expect("Failed to create UDP client");
    
    // Create DataRefReader
    let dataref_reader = DataRefReader::new(&client);
    
    // Read dataref value
    let value = dataref_reader.read("sim/cockpit2/controls/parking_brake_ratio", "float")
        .expect("Failed to read dataref");
    
    println!("Parking brake: {}", value);
}
```

## Protocol

The communication between clients and the plugin follows a simple text-based protocol:

### Request Format
```
dataref|read|<type>|<dataref_name>
```

Where:
- `<type>` can be: `int`, `float`, `[int]`, `[float]`
- `<dataref_name>` is the X-Plane data reference identifier

### Response Format
```
dataref|response|<type>|<value>
```

### Example
Request:
```
dataref|read|float|sim/cockpit2/controls/parking_brake_ratio
```

Response:
```
dataref|response|float|0.0
```

## Supported Data Types

| Type | Description | Example |
|------|-------------|---------|
| `int` | Integer values | `sim/cockpit2/electrical/battery_on` |
| `float` | Floating-point values | `sim/cockpit2/controls/parking_brake_ratio` |
| `[int]` | Integer arrays | `sim/cockpit2/engine/actuators/eng_master` |
| `[float]` | Float arrays | `sim/flightmodel/position/local_x` |

## Common Data References

Here are some commonly used data references to get you started:

- `sim/cockpit2/controls/parking_brake_ratio` - Parking brake (0.0 to 1.0)
- `sim/cockpit2/engine/actuators/throttle_ratio` - Throttle position (0.0 to 1.0)
- `sim/cockpit2/gauges/indicators/altitude_ft_pilot` - Altitude in feet
- `sim/flightmodel/position/latitude` - Latitude in degrees
- `sim/flightmodel/position/longitude` - Longitude in degrees
- `sim/flightmodel/position/true_airspeed` - True airspeed in knots

For a complete list of data references, refer to the [X-Plane Data References Documentation](https://developer.x-plane.com/datarefs/).

## Development

### Building the Plugin

The plugin is built using Rust and requires the following dependencies:

- Rust toolchain (latest stable)
- libclang (for bindgen)
- X-Plane SDK headers

Run the build script:
```bash
cd plugin
build.bat
```

### Building the Clients

#### Python Client
```bash
cd demo/python
uv sync
uv run src/main.py
```

#### Go Client
```bash
cd demo/go
go mod tidy
go run main/main.go
```

#### Rust Client
```bash
cd demo/rust
cargo run
```

## Troubleshooting

### Common Issues

1. **Plugin not loading**: Ensure the plugin is correctly placed in the X-Plane plugins directory
2. **Connection refused**: Verify X-Plane is running and the plugin is loaded
3. **Timeout errors**: Check if the firewall is blocking UDP connections on port 49000

### Debugging

Enable debug logging by checking the log file in the X-Plane installation directory:
```
<X-Plane Directory>/XPlaneUdpBridgePlugin.log
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Guidelines

1. Follow the existing code style
2. Add tests for new functionality
3. Update documentation as needed
4. Ensure all tests pass before submitting

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [X-Plane SDK](https://developer.x-plane.com/sdk/) - For providing the plugin development framework
- [xplm](https://crates.io/crates/xplm) - Rust bindings for the X-Plane SDK
- [Tokio](https://tokio.rs/) - Asynchronous runtime for the UDP server

## Support

If you encounter any issues or have questions, please file an issue on the [GitHub repository](https://github.com/codeboyzhou/xplane-udp-bridge/issues).