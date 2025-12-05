package main

import (
	"fmt"
	"strconv"
	"strings"
)

const InvalidDataRefFloatValue = -666.0

type DataRefReader struct {
	client *UdpClient
}

func NewDataRefReader(client *UdpClient) *DataRefReader {
	return &DataRefReader{
		client: client,
	}
}

func (reader *DataRefReader) ReadAsFloat(dataref string) float32 {
	data := fmt.Sprintf("dataref|read|float|%s", dataref)
	fmt.Printf("➡️  Sending dataref read request: %s\n", data)

	response := reader.client.SendAndRecv([]byte(data))
	if response == nil {
		fmt.Printf("❌ Dataref %s read failed: no response from server\n", dataref)
		return InvalidDataRefFloatValue
	}

	body := string(response)
	fmt.Printf("⬅️  Received dataref read response body: %s\n", body)
	value := strings.Split(body, "|")[2]
	floatValue, err := strconv.ParseFloat(value, 32)
	if err != nil {
		fmt.Printf("❌ Error parsing float value: %v\n", err)
		return InvalidDataRefFloatValue
	}

	fmt.Printf("✅ Dataref %s successfully read as float: %.1f\n", dataref, floatValue)
	return float32(floatValue)
}
