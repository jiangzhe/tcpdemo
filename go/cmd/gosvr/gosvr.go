package main

import (
	"fmt"
	"net"
	"time"
)

func main() {
	listener, err := net.Listen("tcp", "127.0.0.1:8080")
	if err != nil {
		panic(err)
	}
	for {
		conn, err := listener.Accept()
		if err != nil {
			panic(err)
		}
		go handle(conn)
	}
}

func handle(conn net.Conn) {
	var reqs int
	start := time.Now()
	buf := make([]byte, 1)
	for {
		n, err := conn.Read(buf)
		if n == 0 || err != nil {
			dur := time.Since(start)
			fmt.Printf("Closed connection. Received %v requests. Duration %v. QPS %v\n", reqs, dur, float64(reqs)/float64(dur.Milliseconds())*1000.0)
			return
		}
		reqs++
		if _, err = conn.Write(buf); err != nil {
			fmt.Printf("failed to write to socket; err = %v\n", err)
			return
		}
	}
}
