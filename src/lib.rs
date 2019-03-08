#![allow(dead_code)]    // To hush ffi warnings, must be a better way
extern crate libc;
use crate::ffi::*;
use libc::c_char;
use std::ffi::CString;

mod ffi;

pub fn my_fmri() -> Option<String> {
    let len = unsafe { scf_limit(SCF_LIMIT_MAX_FMRI_LENGTH as u32) as usize };
    let mut buf: Vec<c_char> = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr() as *mut libc::c_char;

    let hdl;
    unsafe {
        hdl = scf_handle_create(SCF_VERSION as u64);
        if scf_handle_bind(hdl) != 0 {
            scf_handle_destroy(hdl);
            return None;
        }
    }

    let need;
    unsafe {
        need = scf_myname(hdl, ptr, len);
        scf_handle_unbind(hdl);
        scf_handle_destroy(hdl);
    }
    if need == -1 {
        return None;
    }
    let cstring = unsafe { CString::from_raw(ptr).into_string().unwrap() };
    println!("Before return cstring {}", cstring);
    Some(cstring)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert(1 == 1);
    }
}
