package main

import (
	"errors"
	"fmt"
	"net"
	"strings"
	"time"

	"github.com/fatih/color"
)

type UdpClient struct {
	serverAddr *net.UDPAddr
	connection *net.UDPConn
	timeout    time.Duration
}

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
		var netErr net.Error
		if errors.As(err, &netErr) && netErr.Timeout() {
			color.Red("UDP request timed out after %.0f seconds\n", client.timeout.Seconds())
		} else {
			color.Red("UDP error while reading data: %v\n", err)
		}
		return nil
	}

	return buffer[:size]
}
