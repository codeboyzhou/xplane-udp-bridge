// Package main implements a UDP bridge client for X-Plane data references.
// This module provides functionality to read data references from X-Plane
// through UDP communication.
package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/fatih/color"
)

// InvalidDataRefFloatValue represents the error value returned when a dataref
// read operation fails. This constant is used to distinguish between valid
// zero values and actual read failures.
const InvalidDataRefFloatValue = -666.0

// DataRefReader provides functionality to read data references from X-Plane
// through a UDP client connection. It handles the formatting of requests,
// communication with the server, and parsing of responses.
type DataRefReader struct {
	client *UdpClient
}

// NewDataRefReader creates a new DataRefReader instance with the provided UDP client.
// It initializes the reader with the client that will handle the actual UDP communication.
//
// Parameters:
//   - client: A pointer to an initialized UdpClient that will be used for communication.
//
// Returns:
//   - A pointer to the newly created DataRefReader instance.
func NewDataRefReader(client *UdpClient) *DataRefReader {
	return &DataRefReader{
		client: client,
	}
}

// ReadAsFloat reads a data reference from X-Plane and returns its value as a float32.
// It formats the request, sends it via the UDP client, and parses the response.
//
// The method follows the X-Plane UDP protocol format: "dataref|read|float|{dataref}"
//
// Parameters:
//   - dataref: The string identifier of the X-Plane data reference to read.
//
// Returns:
//   - float32: The parsed float value from the data reference.
//   - If the read operation fails or the response cannot be parsed,
//     returns InvalidDataRefFloatValue (-666.0).
//
// Example:
//
//	reader := NewDataRefReader(udpClient)
//	value := reader.ReadAsFloat("sim/cockpit2/gauges/indicators/airspeed_kt_pilot")
//	if value != InvalidDataRefFloatValue {
//	    fmt.Printf("Airspeed: %.1f knots\n", value)
//	}
func (reader *DataRefReader) ReadAsFloat(dataref string) float32 {
	data := fmt.Sprintf("dataref|read|float|%s", dataref)

	fmt.Println(strings.Repeat("=", 100))
	color.Cyan("Sending dataref read request: %s\n", data)

	response := reader.client.SendAndRecv([]byte(data))
	if response == nil {
		color.Red("Dataref %s read failed: no response from server\n", dataref)
		return InvalidDataRefFloatValue
	}

	body := string(response)
	color.Yellow("Received dataref read response body: %s\n", body)
	value := strings.Split(body, "|")[2]
	floatValue, err := strconv.ParseFloat(value, 32)
	if err != nil {
		color.Red("Error parsing float value: %v\n", err)
		return InvalidDataRefFloatValue
	}

	color.Green("Dataref %s successfully read as float: %.1f\n", dataref, floatValue)
	return float32(floatValue)
}
