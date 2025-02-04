use std::{fmt, ffi::CStr, os::raw::c_int};

use super::sys::wavefront_align_strerror;

#[derive(Debug)]
pub struct WfaError(c_int);

impl fmt::Display for WfaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = unsafe { CStr::from_ptr(wavefront_align_strerror(self.0)) }.to_str().expect("Error message not UTF8");
        write!(f,"{p}")
    }
}

#[derive(Debug)]
pub struct WfaStatus(c_int);

impl fmt::Display for WfaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = unsafe { CStr::from_ptr(wavefront_align_strerror(self.0)) }.to_str().expect("Status message not UTF8");
        write!(f,"{p}")
    }
}

#[inline]
pub fn check_status(s: c_int) -> Result<WfaStatus, WfaError> {
    match s {
        0 | 1 => Ok(WfaStatus(s)),
        _ => Err(WfaError(s))
    }
}
