package main

import "net"

var ReqCount = 200000

func main() {
	conn, err := net.Dial("tcp", "127.0.0.1:8080")
	if err != nil {
		panic(err)
	}
	defer conn.Close()
	buf := make([]byte, 1)
	var n int
	for i := 0; i < ReqCount; i++ {
		_, err := conn.Write(buf)
		if err != nil {
			panic(err)
		}
		n, err = conn.Read(buf)
		if err != nil {
			panic(err)
		}
		if n != 1 {
			panic("read bytes must be 1")
		}
	}
}
