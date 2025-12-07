// Package main implements a UDP bridge client for X-Plane data references.
// This module provides functionality to read data references from X-Plane
// through UDP communication, enabling external applications to access
// flight simulator data in real-time.
package main

import (
	"fmt"
	"strings"

	"github.com/fatih/color"
)

// DataRefReader provides functionality to read data references from X-Plane
// through a UDP client connection. It handles the formatting of requests,
// communication with the server, and parsing of responses.
//
// Data references (datarefs) in X-Plane are string identifiers that point to
// specific simulator variables, such as aircraft position, engine status,
// or instrument readings. This reader enables external applications to
// monitor these values in real-time.
type DataRefReader struct {
	client *UdpClient // UDP client for communication with X-Plane
}

// NewDataRefReader creates a new DataRefReader instance with the provided UDP client.
// It initializes the reader with the client that will handle the actual UDP communication.
//
// This function is the factory for creating DataRefReader instances. The UDP client
// must already be initialized and connected to X-Plane before creating the reader.
//
// Parameters:
//   - client: A pointer to an initialized UdpClient that will be used for communication.
//     The client should be configured to connect to X-Plane's UDP interface.
//
// Returns:
//   - *DataRefReader: A pointer to the newly created DataRefReader instance.
//     The returned reader is ready to use for reading data references.
//
// Example:
//
//	// First create and connect the UDP client
//	client := NewUdpClient("127.0.0.1", 49000, 5)
//	if client == nil {
//	    log.Fatal("Failed to create UDP client")
//	}
//
//	// Then create the dataref reader
//	reader := NewDataRefReader(client)
//	value := reader.Read("sim/cockpit2/controls/parking_brake_ratio", "float")
func NewDataRefReader(client *UdpClient) *DataRefReader {
	return &DataRefReader{
		client: client,
	}
}

// Read reads a data reference from X-Plane and returns its value as a string.
// It formats the request, sends it via the UDP client, and parses the response.
//
// This method provides a generic interface to read any data reference from X-Plane,
// regardless of its data type. The caller specifies the expected data type,
// and the method returns the raw value as a string, which can then be parsed
// into the appropriate type by the caller.
//
// The method follows the X-Plane UDP protocol format: "dataref|read|{type}|{dataref}"
//
// Parameters:
//   - dataref: The string identifier of the X-Plane data reference to read.
//     These identifiers follow X-Plane's naming convention, such as
//     "sim/cockpit2/controls/parking_brake_ratio" or "sim/flightmodel/position/latitude".
//   - dataType: The data type of the data reference. Common values include:
//     "int", "float", "[int]", "[float]".
//     The data type must match the actual type of the data reference.
//
// Returns:
//   - string: The raw value from the data reference as a string.
//     For array types, this will be a comma-separated list of values.
//     For string types, this will be the string value.
//   - Empty string (""): If the read operation fails or no response is received.
//
// Example:
//
//	// Read a float value
//	airspeedStr := reader.Read("sim/cockpit2/gauges/indicators/airspeed_kt_pilot", "float")
//	if airspeedStr != "" {
//		airspeed, err := strconv.ParseFloat(airspeedStr, 32)
//		if err == nil {
//			fmt.Printf("Airspeed: %.1f knots\n", airspeed)
//		}
//	}
//
//	// Read an array of floats
//	enginesStr := reader.Read("sim/flightmodel/engine/ENGN_thro", "float_array")
//	if enginesStr != "" {
//		engines := strings.Split(enginesStr, ",")
//		for i, engine := range engines {
//			throttle, _ := strconv.ParseFloat(engine, 32)
//			fmt.Printf("Engine %d throttle: %.1f%%\n", i+1, throttle*100)
//		}
//	}
func (reader *DataRefReader) Read(dataref, dataType string) string {
	data := fmt.Sprintf("dataref|read|%s|%s", dataType, dataref)

	fmt.Println(strings.Repeat("=", 100))
	color.Cyan("Sending dataref read request: %s\n", data)

	response := reader.client.SendAndRecv([]byte(data))
	if response == nil {
		color.Red("Dataref %s read failed: no response from server\n", dataref)
		return ""
	}

	body := string(response)
	color.Yellow("Received dataref read response body: %s\n", body)
	value := strings.Split(body, "|")[2]
	return value
}
