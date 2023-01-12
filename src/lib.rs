#![feature(vec_into_raw_parts)]
use std::os::raw::c_char;
use std::ffi::{CStr, CString};

use controlpads;

#[repr(C)]
pub struct c_string_vec {
    ptr: *mut *mut c_char,
    len: u64,
    cap: u64,
}


type CError = u64;

// TODO: more errors
pub const SUCCESS: CError = 0;
pub const ERROR_CONTROLPADS: CError = 1;
pub const ERROR_CSTR_TO_STR: CError = 2;


fn rust_to_c_strvec(vec: Vec<String>) -> c_string_vec {
    let mut c_string_ptrs: Vec<*mut c_char> = Vec::new();
    for s in vec {
        let c_string = CString::new(s).unwrap();
        let raw = c_string.into_raw();
        c_string_ptrs.push(raw);
    }
    let (ptr, len, cap) = c_string_ptrs.into_raw_parts();
    c_string_vec {
        ptr: ptr,
        len: len as u64,
        cap: cap as u64,
    }
}


#[no_mangle]
pub extern "C" fn free_strvec(vec: c_string_vec) {
    unsafe {
        // we take back ownership of the array memory so that when we leave
        // this scope that memory is freed
        let ptr_vec = Vec::from_raw_parts(vec.ptr, vec.len as usize, vec.cap as usize);
        for c_string in ptr_vec {
            // we take back ownership of the string memory so that when we leave
            // this scope that memory is freed
            let _ = CString::from_raw(c_string); 
        }
    }
}

#[no_mangle]
pub extern "C" fn clients_changed(did_change: &mut bool) -> CError {
    let result = controlpads::clients_changed();
    match result {
        Ok(x) => {
            *did_change = x;
            SUCCESS
        }
        Err(_) => { // TODO: use error (print it to stderr perhaps)
            ERROR_CONTROLPADS
        }
    }

}

#[no_mangle]
pub extern "C" fn get_client_handles(client_handles: *mut c_string_vec) -> CError {
    let result = controlpads::get_client_handles();
    match result {
        Ok(x) => {
            unsafe {
                *client_handles = rust_to_c_strvec(x);
            }
            SUCCESS
        }
        Err(_) => { // TODO: use error
            ERROR_CONTROLPADS
        }
    }
}

#[no_mangle]
pub extern "C" fn send_message(client: *const c_char, msg: *const c_char) -> CError {
    // TODO: We're copying data to make the String and eventually we should
    //       *not* do that
    // TODO: print along with errors
    unsafe {
        let client_str = match CStr::from_ptr(client).to_str() {
            Ok(ok) => ok,
            Err(_) => {
                return ERROR_CSTR_TO_STR;
            }
        };
        let msg_str = match CStr::from_ptr(msg).to_str() {
            Ok(ok) => ok,
            Err(_) => {
                return ERROR_CSTR_TO_STR;
            }
        };

        match controlpads::send_message(&String::from(client_str), msg_str) {
            Ok(()) => {
                SUCCESS
            }
            Err(_) => {
                ERROR_CONTROLPADS
            }
            
        }
    }
}

#[no_mangle]
pub extern "C" fn get_messages(client: *const c_char, messages: *mut c_string_vec) -> CError {
    unsafe {
        let client_str = match CStr::from_ptr(client).to_str() {
            Ok(ok) => ok,
            Err(_) => {
                return ERROR_CSTR_TO_STR;
            }
        };
        let result = controlpads::get_messages(&String::from(client_str));
        match result {
            Ok(x) => {
                *messages = rust_to_c_strvec(x);
                SUCCESS
            }
            Err(_) => {
                ERROR_CONTROLPADS
            }
        }
    }
}
