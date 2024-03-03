use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Container {
    pub s: String
}

#[no_mangle]
pub unsafe extern "C" fn new_container(s: *const c_char) -> Box<Container> {
    let c_str = CStr::from_ptr(s);
    let container = Container {
        s: String::from(c_str.to_str().unwrap()),
    };
    Box::new(container)
}

#[no_mangle]
pub unsafe extern "C" fn get_info(c: &Container) -> *const c_char {
    // println!("s: {}", c.s.as_str());
    CString::new(c.s.clone().as_str()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn destroy(c: *mut c_char) {
    let _ = unsafe {
        CString::from_raw(c)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
