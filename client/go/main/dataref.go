package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/fatih/color"
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
