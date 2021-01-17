#[cfg(test)]
mod test;
mod types;

use comedilib_sys as ffi;
use failure::{bail, Error};
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_uint, c_double};
use std::convert::TryInto;
pub use types::*;

macro_rules! perror {
    ($str:expr) => {
        unsafe {
            bail!(
                "{}: {}",
                $str,
                CStr::from_ptr(ffi::comedi_strerror(ffi::comedi_errno())).to_str()?
            );
        }
    };
}

macro_rules! getter {
    ($name:ident: $ty:ty) => {
        pub fn $name(&self) -> $ty {
            unsafe { (*self.ptr).$name }
        }
    };
    ($name:ident: enum $ty:ty) => {
        pub fn $name(&self) -> $ty {
            <$ty>::from_repr(unsafe { (*self.ptr).$name }).unwrap()
        }
    };
    ($name:ident) => {
        getter!($name: c_uint);
    };
}

pub struct Range<'a> {
    phantom: PhantomData<&'a ()>,
    ptr: *mut ffi::comedi_range,
}

impl<'a> Range<'a> {
    getter!(max: c_double);
    getter!(min: c_double);
    getter!(unit: enum Unit);
}

pub struct Cmd {
    ptr: *mut ffi::comedi_cmd,
}

impl Cmd {
    getter!(subdev);
    getter!(start_src: enum StartTrigger);
    getter!(start_arg);
    getter!(scan_begin_src: enum ScanBeginTrigger);
    getter!(scan_begin_arg);
    getter!(convert_src: enum ConvertTrigger);
    getter!(convert_arg);
    getter!(scan_end_src: enum ScanEndTrigger);
    getter!(scan_end_arg);
    getter!(stop_src: enum StopTrigger);
    getter!(stop_arg);
    pub fn set_subdev(&mut  self, subdev: c_uint) {
        unsafe { (*self.ptr).subdev = subdev };
    }
    pub fn set_start(&mut self, src: StartTrigger, arg: c_uint) {
        unsafe {
            (*self.ptr).start_src = src.repr();
            (*self.ptr).start_arg = arg;
        }
    }
    pub fn set_scan_begin(&mut self, src: ScanBeginTrigger, arg: c_uint) {
        unsafe {
            (*self.ptr).scan_begin_src = src.repr();
            (*self.ptr).scan_begin_arg = arg;
        }
    }
    pub fn set_convert(&mut self, src: ConvertTrigger, arg: c_uint) {
        unsafe {
            (*self.ptr).convert_src = src.repr();
            (*self.ptr).convert_arg = arg;
        }
    }
    pub fn set_scan_end(&mut self, src: ScanEndTrigger, arg: c_uint) {
        unsafe {
            (*self.ptr).scan_end_src = src.repr();
            (*self.ptr).scan_end_arg = arg;
        }
    }
    pub fn set_stop(&mut self, src: StopTrigger, arg: c_uint) {
        unsafe {
            (*self.ptr).stop_src = src.repr();
            (*self.ptr).stop_arg = arg;
        }
    }
    pub fn set_chanlist(&mut self, chanlist: &[(c_uint, c_uint, ARef)]) {
        let mut packed_chanlist: Vec<c_uint> = chanlist.iter().map(|(chan, rng, aref)| cr_pack(*chan, *rng, *aref)).collect();
        unsafe {
            (*self.ptr).chanlist = packed_chanlist.as_mut_ptr();
            (*self.ptr).chanlist_len = packed_chanlist.len().try_into().unwrap();
        }
    }
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
    pub fn data_read(
        &self,
        subdevice: c_uint,
        channel: c_uint,
        range: c_uint,
        aref: ARef,
    ) -> Result<LSampl, Error> {
        let mut data: LSampl = 0;
        if unsafe {
            ffi::comedi_data_read(self.ptr, subdevice, channel, range, aref.repr(), &mut data)
        } < 0
        {
            perror!("comedi_data_read");
        }
        Ok(data)
    }
    pub fn get_range<'a>(
        &'a self,
        subdevice: c_uint,
        channel: c_uint,
        range: c_uint,
    ) -> Result<Range<'a>, Error> {
        let ptr = unsafe { ffi::comedi_get_range(self.ptr, subdevice, channel, range) };
        if ptr.is_null() {
            perror!("comedi_get_range");
        }
        Ok(Range {
            phantom: PhantomData,
            ptr,
        })
    }
    pub fn get_maxdata(&self, subdevice: c_uint, channel: c_uint) -> Result<LSampl, Error> {
        let maxdata = unsafe { ffi::comedi_get_maxdata(self.ptr, subdevice, channel) };
        if maxdata == 0 {
            perror!("comedi_get_maxval");
        }
        Ok(maxdata)
    }
}

pub fn to_phys(data: LSampl, range: &Range, maxdata: LSampl) -> Result<c_double, Error> {
    let phys = unsafe { ffi::comedi_to_phys(data, range.ptr, maxdata) };
    if phys == c_double::NAN {
        perror!("comedi_to_phys");
    }
    Ok(phys)
}

fn cr_pack(chan: c_uint, rng: c_uint, aref: ARef) -> c_uint {
    ((aref.repr() & 0x3) << 24) | ((rng & 0xff) << 16) | chan
}

pub fn set_global_oor_behavior(behavior: OORBehavior) -> OORBehavior {
    OORBehavior::from_repr(unsafe { ffi::comedi_set_global_oor_behavior(behavior.repr()) }).unwrap()
}

impl Drop for Comedi {
    fn drop(&mut self) {
        unsafe {
            if ffi::comedi_close(self.ptr) < 0 {
                panic!(
                    "comedi_close: {}",
                    CStr::from_ptr(ffi::comedi_strerror(ffi::comedi_errno()))
                        .to_str()
                        .unwrap()
                );
            }
        }
    }
}
