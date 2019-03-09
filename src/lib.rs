#![allow(dead_code)]    // To hush ffi warnings, must be a better way
extern crate libc;
use crate::ffi::*;
use std::ffi::{CStr, CString, NulError};
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

impl From<NulError> for SMFError {
    fn from(error: NulError) -> Self {
        SMFError { smf_error: format!("invalid string: {}", error) }
    }
}

struct SCFHandle {
    handle: *mut scf_handle_t,
    bound: bool,
}

impl SCFHandle {
    fn open() -> Result<SCFHandle> {
        let hdl = unsafe { scf_handle_create(SCF_VERSION as u64) };
        if unsafe { scf_handle_bind(hdl) } != 0 {
            return Err(SMFError::new());
        }
        Ok(SCFHandle {
            handle: hdl,
            bound: true,
        })
    }

    fn close(&mut self) {
        if self.bound {
            unsafe {
                scf_handle_unbind(self.handle);
                scf_handle_destroy(self.handle);
            }
            self.bound = false;
        }
    }
}

impl Drop for SCFHandle {
    fn drop(&mut self) {
        self.close();
    }
}

/// Returns the FMRI of the service under which the calling process is running
/// or an SMFError if not running under a service.
///
/// # Example
///
/// ```
/// use smf::my_fmri;
///
/// match my_fmri() {
///     Ok(fmri) => println!("My SMF instance is {}", fmri),
///     Err(e) => eprintln!("{}", e),
/// }
/// ```
pub fn my_fmri() -> Result<String> {
    let len = unsafe { scf_limit(SCF_LIMIT_MAX_FMRI_LENGTH as u32) as usize };
    let ptr = CString::new(" ".repeat(len)).unwrap().into_raw();

    let hdl = SCFHandle::open()?;

    unsafe {
        let need = scf_myname(hdl.handle, ptr, len);
        if need == -1 {
            Err(SMFError {
                smf_error: "process not associated with a service".to_string()
            })
        } else {
            Ok(CString::from_raw(ptr).into_string().unwrap())
        }
    }
}

/// Returns the state of the serivce as one of `uninitialized`, `maintenance`,
/// `offline`, `disabled`, `online`, or `degraded`.
///
/// # Example
///
/// ```
/// use smf::get_state;
///
/// let fmri = "svc:/system/filesystem/local:default";
/// match get_state(&fmri) {
///    Ok(state) => println!("{} is {}", fmri, state),
///    Err(e) => eprintln!("{} has no state: {}", e),
/// }
/// ```
pub fn get_state(fmri: &str) -> Result<String> {
    let state = unsafe { smf_get_state(CString::new(fmri)?.as_ptr()) };
    if state.is_null() {
        Err(SMFError::new())
    } else {
        Ok(unsafe { CStr::from_ptr(state) }.to_string_lossy().to_string())
    }
}
