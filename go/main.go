package main

//#cgo CFLAGS: -I${SRCDIR}/..
//#cgo LDFLAGS: -Wl,-rpath,${SRCDIR}/.. -L${SRCDIR}/.. -lrustcbinding
//#include <header.h>
import "C"
import "fmt"

func main() {
	// call to create new Container
	container := C.new_container(C.CString("milktea with coffee"), C.uint(124)) //C.add(C.uintptr_t(10), C.uintptr_t(50))

	// read inner value of container, which is the *const char
	containerInfo := C.get_name(container)
	fmt.Println("This is the info:", C.GoString(containerInfo), ", then we are going to destroy this string")
	C.destroy(containerInfo)

	another := C.get_name(container)
	fmt.Println("We can still get the container info normally:", C.GoString(another))

	product := C.new_product(C.ulonglong(124), container)
	fmt.Println("prod:", product)

	container = C.get_container(product) //C.add(C.uintptr_t(10), C.uintptr_t(50))
	productName := C.get_name(container)
	fmt.Println("This is the name of product after we get from product struct:", C.GoString(productName))
}
