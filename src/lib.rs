use comedilib_sys as ffi;
use std::ffi::{CString, CStr};
use std::os::raw::*;
use failure::{bail, Error};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

macro_rules! perror {
    ($str:expr) => {
        unsafe { bail!("{}: {}", $str, CStr::from_ptr(ffi::comedi_strerror(ffi::comedi_errno())).to_str()?); }
    }
}

pub type LSampl = ffi::lsampl_t;
pub type Range = ffi::comedi_range;

#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum AREF {
    GROUND = ffi::AREF_GROUND,
    COMMON = ffi::AREF_COMMON,
    DIFF = ffi::AREF_DIFF,
    OTHER = ffi::AREF_OTHER
}

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum OORBehavior {
    NAN = ffi::comedi_oor_behavior_COMEDI_OOR_NAN,
    NUMBER = ffi::comedi_oor_behavior_COMEDI_OOR_NUMBER,
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
        Ok(Comedi { ptr })
    }
    pub fn data_read(&self, subdevice: c_uint, channel: c_uint, range: c_uint, aref: AREF) -> Result<LSampl, Error> {
        let mut data: LSampl = 0;
        if unsafe { ffi::comedi_data_read(self.ptr, subdevice, channel, range, aref.into(), &mut data) } < 0 {
            perror!("comedi_data_read");
        }
        Ok(data)
    }
    pub fn get_range<'a>(&'a self, subdevice: c_uint, channel: c_uint, range: c_uint) -> Result<&'a mut Range, Error> {
        let range = unsafe { ffi::comedi_get_range(self.ptr, subdevice, channel, range) };
        if range.is_null() {
            perror!("comedi_get_range");
        }
        Ok(unsafe { &mut*range })
    }
    pub fn get_maxdata(&self, subdevice: c_uint, channel: c_uint) -> Result<LSampl,Error> {
        let maxdata = unsafe { ffi::comedi_get_maxdata(self.ptr, subdevice, channel) };
        if maxdata == 0 {
            perror!("comedi_get_maxval");
        }
        Ok(maxdata)
    }
}

pub fn to_phys(data: LSampl, range: &mut Range, maxdata: LSampl) -> Result<c_double,Error> {
    let phys = unsafe { ffi::comedi_to_phys(data, range as *mut Range, maxdata) };
    if phys == c_double::NAN {
        perror!("comedi_to_phys");
    }
    Ok(phys)
}

pub fn set_global_oor_behavior(behavior: OORBehavior) -> OORBehavior {
    OORBehavior::try_from(unsafe { ffi::comedi_set_global_oor_behavior(behavior.into()) }).unwrap()
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
