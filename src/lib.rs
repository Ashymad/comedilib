use comedilib_sys as ffi;
use std::ffi::{CString, CStr};
use std::os::raw::*;
use failure::{bail, Error};
use num_enum::IntoPrimitive;

macro_rules! perror {
    ($str:expr) => {
        unsafe { bail!("{}: {}", $str, CStr::from_ptr(ffi::comedi_strerror(ffi::comedi_errno())).to_str()?); }
    }
}

type LSampl = ffi::lsampl_t;

#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum AREF {
    GROUND = ffi::AREF_GROUND,
    COMMON = ffi::AREF_COMMON,
    DIFF = ffi::AREF_DIFF,
    OTHER = ffi::AREF_OTHER
}

pub struct Comedi {
    ptr: *mut ffi::comedi_t,
}

impl Comedi {
    pub fn open(index: u8) -> Result<Self, Error> {
        let dev = CString::new("/dev/comedi".to_owned() + &index.to_string()).unwrap();
        let ptr = unsafe { ffi::comedi_open(dev.as_ptr()) };
        if ptr.is_null() {
            perror!(format!("comedi_open({})", dev.into_string().unwrap()));
        }
        return Ok(Comedi { ptr });
    }
    pub fn data_read(&self, subdevice: c_uint, channel: c_uint, range: c_uint, aref: AREF) -> Result<LSampl, Error> {
        let mut data: LSampl = 0;
        if unsafe { ffi::comedi_data_read(self.ptr, subdevice, channel, range, aref.into(), &mut data) } < 0 {
            perror!("comedi_data_read");
        }
        return Ok(data);
    }
}

impl Drop for Comedi {
    fn drop(&mut self) {
        unsafe { if ffi::comedi_close(self.ptr) < 0 {
            panic!("comedi_close: {}", CStr::from_ptr(ffi::comedi_strerror(ffi::comedi_errno())).to_str().unwrap());
        }}
    }
}

#[cfg(test)]
mod test;
