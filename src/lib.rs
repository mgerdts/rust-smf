#![allow(dead_code)]    // To hush ffi warnings, must be a better way
extern crate libc;
use crate::ffi::*;
use std::ffi::{CStr, CString};
use std::error;
use std::fmt;
use std::error::Error;

type Result<T> = std::result::Result<T, SMFError>;

mod ffi;

#[derive(Debug)]
pub struct SMFError {
    smf_error: String,
}

impl SMFError {
    fn new() -> SMFError {
        SMFError {
            smf_error: unsafe {
                CStr::from_ptr(scf_strerror(scf_error()))
                    .to_str().unwrap().to_owned()
            },
        }
    }
}

impl fmt::Display for SMFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMF Error: {}", self.smf_error)
    }
}


impl Error for SMFError {
    fn description(&self) -> &str {
        &self.smf_error
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub fn my_fmri() -> Result<String> {
    let len = unsafe { scf_limit(SCF_LIMIT_MAX_FMRI_LENGTH as u32) as usize };
    let ptr = CString::new(" ".repeat(len)).unwrap().into_raw();

    unsafe {
        let hdl = scf_handle_create(SCF_VERSION as u64);
        if scf_handle_bind(hdl) != 0 {
            scf_handle_destroy(hdl);
            return Err(SMFError::new());
        }
        let need = scf_myname(hdl, ptr, len);
        scf_handle_unbind(hdl);
        scf_handle_destroy(hdl);
        if need == -1 {
            return Err(SMFError::new());
        }
        Ok(CString::from_raw(ptr).into_string().unwrap())
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
