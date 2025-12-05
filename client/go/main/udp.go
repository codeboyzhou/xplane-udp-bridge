package main

import (
	"errors"
	"fmt"
	"net"
	"time"
)

type UdpClient struct {
	serverAddr *net.UDPAddr
	connection *net.UDPConn
	timeout    time.Duration
}

func NewUdpClient(host string, port, timeoutSecs int) *UdpClient {
	fmt.Printf("🔗 Connecting to %s:%d with timeout %d seconds\n", host, port, timeoutSecs)

	serverAddr := &net.UDPAddr{
		IP:   net.ParseIP(host),
		Port: port,
	}
	timeout := time.Duration(timeoutSecs) * time.Second

	connection, err := net.DialUDP("udp", nil, serverAddr)
	if err != nil {
		fmt.Printf("❌ UDP error while connecting to server: %v\n", err)
		return nil
	}

	fmt.Println("✅ Connected successfully via UDP protocol")

	return &UdpClient{
		serverAddr: serverAddr,
		connection: connection,
		timeout:    timeout,
	}
}

func (client *UdpClient) SendAndRecv(data []byte) []byte {
	_, err := client.connection.Write(data)
	if err != nil {
		fmt.Printf("❌ UDP error while sending data: %v\n", err)
		return nil
	}

	err = client.connection.SetReadDeadline(time.Now().Add(client.timeout))
	if err != nil {
		fmt.Printf("❌ UDP error while setting read deadline: %v\n", err)
		return nil
	}

	buffer := make([]byte, 2048)

	size, _, err := client.connection.ReadFromUDP(buffer)
	if err != nil {
		var netErr net.Error
		if errors.As(err, &netErr) && netErr.Timeout() {
			fmt.Printf("⏰ UDP request timed out after %.0f seconds\n", client.timeout.Seconds())
		} else {
			fmt.Printf("❌ UDP error while reading data: %v\n", err)
		}
		return nil
	}

	return buffer[:size]
}
