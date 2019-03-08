#![allow(dead_code)]    // To hush ffi warnings, must be a better way
extern crate libc;
use crate::ffi::*;
use std::ffi::CString;

mod ffi;

pub fn my_fmri() -> Option<String> {
    let len = unsafe { scf_limit(SCF_LIMIT_MAX_FMRI_LENGTH as u32) as usize };
    let ptr = CString::new(format!("{:len$}", " ", len = len)).unwrap()
        .into_raw();

    unsafe {
        let hdl = scf_handle_create(SCF_VERSION as u64);
        if scf_handle_bind(hdl) != 0 {
            scf_handle_destroy(hdl);
            return None;
        }
        let need = scf_myname(hdl, ptr, len);
        scf_handle_unbind(hdl);
        scf_handle_destroy(hdl);
        if need == -1 {
            return None;
        }
        Some(CString::from_raw(ptr).into_string().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert(1 == 1);
    }
}
