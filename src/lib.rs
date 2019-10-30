#![allow(dead_code)] // To hush ffi warnings, must be a better way
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate libc;
use crate::ffi::*;
use libc::timeval;
use std::error::Error;
use std::ffi::{CStr, CString, NulError};
use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};

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
                    .to_str()
                    .unwrap()
                    .to_owned()
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

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<NulError> for SMFError {
    fn from(error: NulError) -> Self {
        SMFError {
            smf_error: format!("invalid string: {}", error),
        }
    }
}

pub struct SCFHandle {
    handle: *mut scf_handle_t,
    bound: bool,
}

impl SCFHandle {
    pub fn open() -> Result<SCFHandle> {
        let hdl = unsafe { scf_handle_create(SCF_VERSION as u64) };
        if unsafe { scf_handle_bind(hdl) } != 0 {
            return Err(SMFError::new());
        }
        Ok(SCFHandle {
            handle: hdl,
            bound: true,
        })
    }

    pub fn close(&mut self) {
        if self.bound {
            println!("closing");
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
        println!("dropping");
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
                smf_error: "process not associated with a service".to_string(),
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
///    Err(e) => eprintln!("{} has no state: {}", fmri, e),
/// }
/// ```
pub fn get_state(fmri: &str) -> Result<String> {
    let state = unsafe { smf_get_state(CString::new(fmri)?.as_ptr()) };
    if state.is_null() {
        Err(SMFError::new())
    } else {
        Ok(unsafe { CStr::from_ptr(state) }
            .to_string_lossy()
            .to_string())
    }
}

// SMF allows the following types:
//  boolean
//  count
//  integer
//  time
//  astring
//  opaque
//  ustring
//  uri
//  fmri
//  host
//  hostname
//  net_address
//  net_address_v4
//  net_address_v6

pub trait Property {
    fn get_value<T>(&self) -> T;

    fn to_string(&self) {
        return self.get_value().to_string();
    }
}

pub struct BoolProp {
    // XXX remove pub
    pub inner: bool,
}

impl Property for BoolProp {
    fn get_value<bool>(&self) -> bool {
        return self.inner;
    }
}

pub struct CountProp {
    inner: u64,
}

pub struct IntegerProp {
    inner: i64,
}

pub struct TimeProp {
    inner: timeval,
}

// ASCII String
pub struct AStringProp {
    pub inner: String,
}

// utf8 string
pub struct UStringProp {
    inner: String,
}

pub struct OpaqueProp {
    inner: Vec<u8>,
}

pub struct UriProp {
    inner: String,
}

pub struct FmriProp {
    inner: String,
}

// May be a HostnameProp or a NetAdressProp
pub struct HostProp {
    inner: String,
}

// A valid utf8 string, presumably representing a hostname.
pub struct HostnameProp {
    inner: String,
}

// NetAddressV4Prop or NetAddressV6Prop
pub struct NetAddressProp {
    inner: String, // FIXME
}

pub struct NetAddressV4Prop {
    inner: Ipv4Addr,
}

pub struct NetAddressV6Prop {
    inner: Ipv6Addr,
}

pub enum PropertyValue {
    Bool(BoolProp),
    Count(CountProp),
    Integer(IntegerProp),
    Time(TimeProp),
    AString(AStringProp),
    UString(UStringProp),
    Opaque(OpaqueProp),
    Uri(UriProp),
    Fmri(FmriProp),
    Host(HostProp),
    Hostname(HostnameProp),
    NetAddress(NetAddressProp),
    NetAddressV4(NetAddressV4Prop),
    NetAddressV6(NetAddressV6Prop),
    BoolList(Vec<BoolProp>),
    CountList(Vec<CountProp>),
    IntegerList(Vec<IntegerProp>),
    TimeList(Vec<TimeProp>),
    AStringList(Vec<AStringProp>),
    UStringList(Vec<UStringProp>),
    OpaqueList(Vec<OpaqueProp>),
    UriList(Vec<UriProp>),
    FmriList(Vec<FmriProp>),
    HostList(Vec<HostProp>),
    HostnameList(Vec<HostnameProp>),
    NetAddressList(Vec<NetAddressProp>),
    NetAddressV4List(Vec<NetAddressV4Prop>),
    NetAddressV6List(Vec<NetAddressV6Prop>),
}

/// Get the composed value of a property.
///
/// If making many calls, use / `SCFHandle::new()` to get a handle and pass via
/// `Some(hdl)`.  For quick one-off calls, passing `None` as `hdl` is just as
/// efficient.
///
/// # Examples
///
/// Get a single property
///
/// ```
/// use smf::{PropertyValue, PropGetOne};
///
/// let fmri = "svc:/network/ssh:default".to_string();
/// match PropGetOne(None, &fmri, "general", "action_authorization").unwrap() {
///     PropertyValue::AString(auth) => {
///         println!("{} can be managed by users with {} authorization",
///             fmri, auth.inner);
///     },
///     _ => println!("Gotta be root to manage {}", fmri),
/// }
/// ```
///
/// Get multiple properties with the same handle.
///
/// ```
/// use smf::{PropertyValue, PropGetOne, SCFHandle};
///
/// let fmri = "svc:/network/ssh:default".to_string();
/// let hdl = SCFHandle::open().unwrap();
///
/// for pg in vec!["start".to_string(), "stop".to_string()].iter() {
///     match PropGetOne(Some(&hdl), &fmri, &pg, "exec").unwrap() {
///         PropertyValue::AString(cmd) => {
///             println!("{} {} command: {}", fmri, pg, cmd.inner);
///         },
///         _ => {},
///     }
/// }
/// ```
pub fn PropGetOne(
    hdl: Option<&SCFHandle>,
    fmri: &str,
    pg: &str,
    prop: &str,
) -> Result<PropertyValue> {
    let hdlholder;
    let hdl = match hdl {
        Some(h) => h,
        None => {
            println!("opening handle");
            hdlholder = SCFHandle::open()?;
            &hdlholder
        },
    };
    let handle = &hdl.handle;

    let propvals = unsafe {
        scf_simple_prop_get(
            *handle,
            CString::new(fmri)?.as_ptr(),
            CString::new(pg)?.as_ptr(),
            CString::new(prop)?.as_ptr(),
        )
    };
    if propvals.is_null() {
        return Err(SMFError::new());
    }

    let numvals = unsafe { scf_simple_prop_numvalues(propvals) };
    if numvals != 1 {
        return Err(SMFError {
            smf_error: format!(
                "{} {}/{} expected 1 propval, got {}",
                fmri, pg, prop, numvals
            ),
        });
    }

    let proptype = unsafe { scf_simple_prop_type(propvals) };

    Ok(match proptype {
        scf_type_t_SCF_TYPE_ASTRING => unsafe {
            let val = scf_simple_prop_next_astring(propvals);
            PropertyValue::AString(AStringProp {
                inner: CStr::from_ptr(val).to_string_lossy().to_string(),
            })
        },
        _ => {
            return Err(SMFError {
                smf_error: "not implemented".to_string(),
            })
        }
    })
}
