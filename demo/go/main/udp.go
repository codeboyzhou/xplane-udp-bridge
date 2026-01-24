// Package main implements a UDP bridge client for X-Plane data references.
// This module provides functionality to communicate with X-Plane through UDP.
package main

import (
	"fmt"
	"net"
	"strings"
	"time"

	"github.com/fatih/color"
)

// UdpClient represents a UDP client for communicating with X-Plane.
// It encapsulates the connection details, server address, and timeout configuration
// for sending requests and receiving responses.
type UdpClient struct {
	serverAddr *net.UDPAddr  // The UDP address of the X-Plane server
	connection *net.UDPConn  // The UDP connection for communication
	timeout    time.Duration // Timeout duration for read operations
}

// NewUdpClient creates a new UDP client for communicating with X-Plane.
// It establishes a UDP connection to the specified server with the given timeout.
//
// Parameters:
//   - host: The IP address or hostname of the X-Plane server.
//   - port: The port number on which X-Plane is listening for UDP connections.
//   - timeoutSecs: The timeout in seconds for read operations.
//
// Returns:
//   - *UdpClient: A pointer to the newly created UDP client.
//   - nil: If the connection could not be established.
//
// Example:
//
//	client := NewUdpClient("127.0.0.1", 49000, 5)
//	if client != nil {
//	    // Use client for communication
//	}
func NewUdpClient(host string, port, timeoutSecs int) *UdpClient {
	fmt.Println(strings.Repeat("=", 100))
	color.Cyan("Creating UDP client to server %s:%d with timeout %d seconds\n", host, port, timeoutSecs)

	serverAddr := &net.UDPAddr{
		IP:   net.ParseIP(host),
		Port: port,
	}
	timeout := time.Duration(timeoutSecs) * time.Second

	connection, err := net.DialUDP("udp", nil, serverAddr)
	if err != nil {
		color.Red("UDP error while creating client: %v\n", err)
		return nil
	}

	color.Green("Created UDP client successfully\n")

	return &UdpClient{
		serverAddr: serverAddr,
		connection: connection,
		timeout:    timeout,
	}
}

// SendAndRecv sends data to the X-Plane server and waits for a response.
// It handles the complete request-response cycle with proper timeout handling.
//
// The method:
// 1. Sends the provided data to the connected server
// 2. Sets a read deadline based on the client's timeout configuration
// 3. Waits for a response up to the specified timeout
// 4. Returns the received data or nil if an error occurs
//
// Parameters:
//   - data: The byte slice containing the data to send to X-Plane.
//
// Returns:
//   - []byte: The response received from X-Plane.
//   - nil: If an error occurs during sending or receiving.
//
// Note:
//
//	The method uses a 2048-byte buffer for receiving data, which is
//	sufficient for typical X-Plane data reference responses.
func (client *UdpClient) SendAndRecv(data []byte) []byte {
	_, err := client.connection.Write(data)
	if err != nil {
		color.Red("UDP error while sending data: %v\n", err)
		return nil
	}

	err = client.connection.SetReadDeadline(time.Now().Add(client.timeout))
	if err != nil {
		color.Red("UDP error while setting read deadline: %v\n", err)
		return nil
	}

	buffer := make([]byte, 2048)

	size, _, err := client.connection.ReadFromUDP(buffer)
	if err != nil {
		color.Red("UDP error while reading data: %v\n", err)
		return nil
	}

	return buffer[:size]
}
