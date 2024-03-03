package main

//#cgo CFLAGS: -I${SRCDIR}/..
//#cgo LDFLAGS: -Wl,-rpath,${SRCDIR}/.. -L${SRCDIR}/.. -lrustcbinding
//#include <header.h>
import "C"
import "fmt"

func main() {
	// call to create new Container
	container := C.new_container(C.CString("haha")) //C.add(C.uintptr_t(10), C.uintptr_t(50))

	// read inner value of container, which is the *const char
	containerInfo := C.get_info(container)
	fmt.Println("This is the info:", C.GoString(containerInfo), ", then we are going to destroy this string")
	C.destroy(containerInfo)

	another := C.get_info(container)
	fmt.Println("We can still get the container info normally:", C.GoString(another))
}
