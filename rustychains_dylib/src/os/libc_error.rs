use libc::c_int;
use std::io::{Error, Result};

pub fn libc_err(num: c_int) -> Result<()> {
    if num < 0 {
        return Err(Error::last_os_error());
    }

    Ok(())
}
