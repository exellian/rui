use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub fn encode_wide(str: impl AsRef<OsStr>) -> Vec<u16> {
    str.as_ref().encode_wide().collect()
}