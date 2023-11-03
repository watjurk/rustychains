package main

import (
	"io"
	"net"
)

func main() {
	listener, err := net.Listen("tcp4", ":8765")
	if err != nil {
		panic(err)
	}

	conn, err := listener.Accept()
	if err != nil {
		panic(err)
	}

	_, err = io.Copy(conn, conn)
	if err != nil {
		panic(err)
	}
}
