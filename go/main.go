package main

//#cgo CFLAGS: -I${SRCDIR}/..
//#cgo LDFLAGS: -Wl,-rpath,${SRCDIR}/.. -L${SRCDIR}/.. -lrustcbinding
//#include <header.h>
import "C"
import "fmt"

func main() {
	container := C.new_container(C.CString("haha")) //C.add(C.uintptr_t(10), C.uintptr_t(50))
	containerInfo := C.get_info(container)
	fmt.Println("result:", C.GoString(containerInfo))
	C.destroy(containerInfo)
	fmt.Println("result:", C.GoString(containerInfo))
}
