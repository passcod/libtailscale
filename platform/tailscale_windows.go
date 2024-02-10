package platform

//#include "errno.h"
import "C"

import (
	"syscall"
)

// simulate socketpair by creating two connected IPv4 sockets at a random port
func GetSocketPair() ([2]int, error) {
	listen_sock, err = syscall.Socket(syscall.AF_INET, syscall.SOCK_STREAM, 0)
	if err != nil {
		return nil, err
	}

	err = syscall.Bind(listen_sock, &syscall.SockaddrInet4{Port: 0, Addr: [4]byte{127, 0, 0, 1}})
	if err != nil {
		return nil, err
	}

	err = syscall.Listen(listen_sock, 1)
	if err != nil {
		return nil, err
	}

	// get the effective port number
	sockaddr, err = syscall.GetSockName(listen_sock)
	if err != nil {
		return nil, err
	}

	client_sock, err = syscall.Socket(syscall.AF_INET, syscall.SOCK_STREAM, 0)
	if err != nil {
		return nil, err
	}

	err = syscall.Connect(client_sock, sockaddr)
	if err != nil {
		return nil, err
	}

	send_sock, _, err = syscall.Accept(listen_sock)
	if err != nil {
		return nil, err
	}

	err = syscall.Close(listen_sock)
	if err != nil {
		return nil, err
	}

	return [2]int{send_sock, client_sock}, nil
}

func CloseSocket(fd int) error {
	err := syscall.Close(fd)
	errCode := syscall.GetLastError()
	return err
}

func ReadSocket(fd int, buf *[256]byte) {
	syscall.Read(fd, (*buf)[:])
}

func SendMessage(fd int, p []byte, connFd int, to syscall.Sockaddr, flags int) error {
	_, err := syscall.Write(fd, p)
	return err
}

func Shutdown(fd int, how int) error {
	return syscall.Shutdown(fd, how)
}
