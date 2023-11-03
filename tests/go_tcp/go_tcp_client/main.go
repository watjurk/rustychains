package main

import (
	"fmt"
	"net"
)

func main() {
	conn, err := net.Dial("tcp4", "localhost:8765")
	if err != nil {
		panic(err)
	}

	message := "Test message\n"
	fmt.Print("Send: ", message)
	_, err = conn.Write([]byte(message))
	if err != nil {
		panic(err)
	}

	recv_message_bytes := make([]byte, len(message))
	_, err = conn.Read(recv_message_bytes)
	if err != nil {
		panic(err)
	}

	fmt.Print("Received: ", string(recv_message_bytes))
}
