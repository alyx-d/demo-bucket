package main

import (
	"fmt"
	"net"
	"strings"
	"time"
)

func main() {
	localIP := GetLocalIPs()
	fmt.Println("Local IP:", localIP)
	ScanLocalPorts()
}

func GetLocalIPs() []string {
	addrs, err := net.InterfaceAddrs()
	result := []string{}
	if err != nil {
		return result
	}
	for _, addr := range addrs {
		if ipnet, ok := addr.(*net.IPNet); ok && !ipnet.IP.IsLoopback() {
			if ipnet.IP.To4() != nil && !strings.HasPrefix(ipnet.IP.String(), "169") {
				result = append(result, ipnet.IP.String())
			}
		}
	}
	return result
}

func ScanLocalPorts() {
	for i := 1; i < 65535; i++ {
		_, err := net.DialTimeout("tcp", fmt.Sprintf("%s:%d", "127.0.0.1", i), time.Second)
		if err == nil {
			fmt.Printf("Port %d is open\n", i)
		}
	}
}
