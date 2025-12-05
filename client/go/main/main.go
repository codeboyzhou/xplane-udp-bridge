// Package main implements a UDP bridge client for X-Plane data references.
// This executable demonstrates how to use the UDP client and DataRefReader
// to continuously monitor data references from X-Plane.
package main

import (
	"time"
)

// main is the entry point of the X-Plane UDP bridge client application.
// It demonstrates the usage of the UDP client and DataRefReader to
// continuously read data references from X-Plane.
//
// The application:
// 1. Creates a UDP client connection to X-Plane at 127.0.0.1:49000 with a 3-second timeout
// 2. Initializes a DataRefReader with the UDP client
// 3. Continuously reads the parking brake ratio data reference
// 4. Sleeps for 3 seconds between iterations to avoid server overload
//
// Note:
//
//	This is a demonstration application. In a production environment,
//	you would typically implement proper error handling and graceful shutdown.
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
