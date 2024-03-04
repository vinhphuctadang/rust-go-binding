use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/*
    Go with this stupid design
    Product => {
        date u64
        container *Container {
            product_name String
            quantity u32
        }
    }
 */
#[no_mangle]
pub unsafe extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Container {
    pub product_name: String,
    pub quantity: u32,
}

pub struct Product {
    pub date: u64,
    pub container: Box<Container>,
}

#[no_mangle]
pub unsafe extern "C" fn new_container(prod_name: *const c_char, quantity: u32) -> Box<Container> {
    let c_str = CStr::from_ptr(prod_name);
    let container = Container {
        product_name: String::from(c_str.to_str().unwrap()),
        quantity: quantity,
    };
    
    Box::new(container)
}

#[no_mangle]
pub unsafe extern "C" fn get_name(c: &Container) -> *const c_char {
    CString::new(c.product_name.clone().as_str()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn destroy(s: *mut c_char) {
    let _ = CString::from_raw(s);
}

#[no_mangle]
pub unsafe extern "C" fn get_quantity(c: &Container) -> u32 {
    c.quantity
}

#[no_mangle]
pub unsafe extern "C" fn new_product(date: u64, product_container: Box<Container>) -> Box<Product> {
    Box::new(Product{
        date,
        container: product_container,
    })
}

// paste it back to rust and let it deallocate
#[no_mangle]
pub unsafe extern "C" fn destroy_product(_: Box<Product>) {
}

#[no_mangle]
pub unsafe extern "C" fn get_container(product: &Product) -> &Container {
    &product.container
}
