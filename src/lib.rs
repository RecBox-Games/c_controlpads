#![feature(vec_into_raw_parts)]
use std::os::raw::c_char;
use std::ffi::CString;

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
pub const FAILURE: CError = 1;


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
        Err(_) => { // TODO: use error
            FAILURE
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
            FAILURE
        }
    }

}
/*
/// Send an atomic message to the specified control pad client
pub fn send_message(client: &ClientHandle, msg: &str) -> Result<()> {
    let ipc_name = client.to_string() + "_out";
    println!("sent {}", msg);
    let delin_msg = msg.to_string() + str::from_utf8(&[0])?;
    ipc::write(&ipc_name, &delin_msg)
	.unwrap_or_else(|e| panic!("Failed to write: {}", e));
    Ok(())
}

/// Returns a vector of all messages that have been received from the
/// specified control pad client since the last call to this function for that
/// client
pub fn get_messages(client: &ClientHandle) -> Result<Vec<String>> {
    let mut ret: Vec<String> = Vec::new();
    let ipc_name = client.to_string() + "_in";
    let msgs_string = ipc::consume(&ipc_name)
	.unwrap_or_else(|e| panic!("Failed to consume: {}", e));
    if msgs_string.len() == 0 {
	return Ok(vec![]);
    }
    println!("{}", msgs_string.replace(str::from_utf8(&[0])?, "0"));
    let mut parts = msgs_string.split(str::from_utf8(&[0])?).collect::<Vec<&str>>();
    parts.pop(); // there will be nothing after last null byte
    for p in &parts {
	//println!("got {}", p);
	ret.push(String::from(*p));
    }
    Ok(ret)
}
*/
