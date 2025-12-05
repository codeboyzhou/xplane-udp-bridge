package main

import (
	"time"
)

func main() {
	// Create UDP client
	client := NewUdpClient("127.0.0.1", 49000, 3)

	// Create DataRefReader
	reader := NewDataRefReader(client)

	for {
		// Read dataref value examples
		datarefs := []string{
			"sim/cockpit2/controls/parking_brake_ratio",
		}

		for _, dataref := range datarefs {
			reader.ReadAsFloat(dataref)
		}

		// Sleep for a short duration to avoid overloading the server
		time.Sleep(time.Duration(3) * time.Second)
	}
}
