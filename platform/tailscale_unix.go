//go:build darwin || linux
// +build darwin linux

package platform

//#include "errno.h"
import "C"

import (
	"syscall"
)

func GetSocketPair() ([2]int, error) {
	return syscall.Socketpair(syscall.AF_LOCAL, syscall.SOCK_STREAM, 0)
}

func CloseSocket(fd int) (err error) {
	return syscall.Close(fd)
}

func ReadSocket(fd int, buf *[256]byte) {
	syscall.Read(fd, (*buf)[:])
}

func SendMessage(fd int, p []byte, connFd int, to syscall.Sockaddr, flags int) (err error) {
	rights := syscall.UnixRights(connFd)
	return syscall.Sendmsg(fd, p, rights, to, flags)
}

func Shutdown(fd int, how int) (err error) {
	return syscall.Shutdown(fd, how)
}
