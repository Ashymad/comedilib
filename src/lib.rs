#[cfg(test)]
mod test;
mod types;
mod flags;

use comedilib_sys as ffi;
use failure::{bail, Error};
use nix::fcntl::{flock, FlockArg};
use std::cell::UnsafeCell;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::{c_double, c_uint};
pub use types::*;
pub use flags::*;

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
            unsafe { (*self.ptr.get()).$name }
        }
    };
    ($name:ident: enum $ty:ty) => {
        pub fn $name(&self) -> $ty {
            unsafe {
                <$ty>::from_repr((*self.ptr.get()).$name).expect(&format!(
                    "Couldn't convert {} to enum of type {}",
                    (*self.ptr.get()).$name,
                    stringify!($ty)
                ))
            }
        }
    };
    ($name:ident) => {
        getter!($name: c_uint);
    };
}

pub struct Range {
    ptr: UnsafeCell<ffi::comedi_range>,
}

impl Range {
    getter!(max: c_double);
    getter!(min: c_double);
    getter!(unit: enum Unit);
}

pub struct Cmd {
    ptr: UnsafeCell<ffi::comedi_cmd>,
    chanlist: Vec<c_uint>,
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
    pub fn set_subdev(&mut self, subdev: c_uint) {
        unsafe { (*self.ptr.get()).subdev = subdev };
    }
    pub fn set_start(&mut self, src: StartTrigger, arg: c_uint) {
        unsafe {
            (*self.ptr.get()).start_src = src.repr();
            (*self.ptr.get()).start_arg = arg;
        }
    }
    pub fn set_scan_begin(&mut self, src: ScanBeginTrigger, arg: c_uint) {
        unsafe {
            (*self.ptr.get()).scan_begin_src = src.repr();
            (*self.ptr.get()).scan_begin_arg = arg;
        }
    }
    pub fn set_convert(&mut self, src: ConvertTrigger, arg: c_uint) {
        unsafe {
            (*self.ptr.get()).convert_src = src.repr();
            (*self.ptr.get()).convert_arg = arg;
        }
    }
    pub fn set_scan_end(&mut self, src: ScanEndTrigger, arg: c_uint) {
        unsafe {
            (*self.ptr.get()).scan_end_src = src.repr();
            (*self.ptr.get()).scan_end_arg = arg;
        }
    }
    pub fn set_stop(&mut self, src: StopTrigger, arg: c_uint) {
        unsafe {
            (*self.ptr.get()).stop_src = src.repr();
            (*self.ptr.get()).stop_arg = arg;
        }
    }
    pub fn set_chanlist(&mut self, chanlist: &[(c_uint, c_uint, ARef)]) {
        self.chanlist = chanlist
            .iter()
            .map(|(chan, rng, aref)| cr_pack(*chan, *rng, *aref))
            .collect();
        unsafe {
            (*self.ptr.get()).chanlist = self.chanlist.as_mut_ptr();
            (*self.ptr.get()).chanlist_len = chanlist.len().try_into().unwrap();
        }
    }
    pub fn chanlist(&self) -> Option<Vec<(c_uint, c_uint, ARef)>> {
        unsafe {
            if (*self.ptr.get()).chanlist.is_null() {
                None
            } else {
                Some(
                    std::slice::from_raw_parts::<c_uint>(
                        (*self.ptr.get()).chanlist,
                        (*self.ptr.get()).chanlist_len.try_into().unwrap(),
                    )
                    .iter()
                    .map(|cr| cr_unpack(*cr))
                    .collect(),
                )
            }
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
        flock(unsafe { ffi::comedi_fileno(ptr) }, FlockArg::LockExclusive)?;
        Ok(Comedi { ptr })
    }
    pub fn data_read(
        &mut self,
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
    pub fn data_read_n(
        &mut self,
        subdevice: c_uint,
        channel: c_uint,
        range: c_uint,
        aref: ARef,
        data: &mut [LSampl],
    ) -> Result<(), Error> {
        if unsafe {
            ffi::comedi_data_read_n(
                self.ptr,
                subdevice,
                channel,
                range,
                aref.repr(),
                data.as_mut_ptr(),
                data.len().try_into().unwrap(),
            )
        } < 0
        {
            perror!("comedi_data_read_n");
        }
        Ok(())
    }
    pub fn get_range(
        &self,
        subdevice: c_uint,
        channel: c_uint,
        range: c_uint,
    ) -> Result<Range, Error> {
        let ptr = unsafe { ffi::comedi_get_range(self.ptr, subdevice, channel, range) };
        if ptr.is_null() {
            perror!("comedi_get_range");
        }
        Ok(Range {
            ptr: UnsafeCell::new(unsafe { *ptr }.clone()),
        })
    }
    pub fn get_maxdata(&self, subdevice: c_uint, channel: c_uint) -> Result<LSampl, Error> {
        let maxdata = unsafe { ffi::comedi_get_maxdata(self.ptr, subdevice, channel) };
        if maxdata == 0 {
            perror!("comedi_get_maxval");
        }
        Ok(maxdata)
    }
    pub fn get_cmd_generic_timed(
        &self,
        subdevice: c_uint,
        chanlist_len: c_uint,
        scan_period_ns: c_uint,
    ) -> Result<Cmd, Error> {
        let cmd = Cmd {
            ptr: unsafe { UnsafeCell::new(std::mem::zeroed::<ffi::comedi_cmd>()) },
            chanlist: Vec::new(),
        };
        if unsafe {
            ffi::comedi_get_cmd_generic_timed(
                self.ptr,
                subdevice,
                cmd.ptr.get(),
                chanlist_len,
                scan_period_ns,
            )
        } < 0
        {
            perror!("comedi_get_cmd_generic_timed");
        }
        Ok(cmd)
    }
    pub fn command_test(&self, cmd: &mut Cmd) -> Result<CommandTestResult, Error> {
        let ret = unsafe { ffi::comedi_command_test(self.ptr, cmd.ptr.get()) };
        if ret < 0 {
            perror!("comedi_command_test");
        }
        Ok(CommandTestResult::from_repr(ret).unwrap())
    }
    pub fn command(&mut self, cmd: &Cmd) -> Result<(), Error> {
        let ret = unsafe { ffi::comedi_command(self.ptr, cmd.ptr.get()) };
        if ret < 0 {
            perror!("comedi_command");
        }
        Ok(())
    }
    pub fn get_subdevice_flags(&self, subdevice: c_uint) -> Result<SDF, Error> {
        let ret = unsafe { ffi::comedi_get_subdevice_flags(self.ptr, subdevice) };
        if ret < 0 {
            perror!("comedi_get_subdevice_flags");
        }
        Ok(SDF::new(ret.try_into().unwrap()))
    }
    pub fn get_read_subdevice(&self) -> Option<c_uint> {
        let ret = unsafe { ffi::comedi_get_read_subdevice(self.ptr) };
        if ret < 0 {
            None
        } else {
            Some(ret.try_into().unwrap())
        }
    }
    pub fn set_read_subdevice(&mut self, subdevice: c_uint) -> Result<(), Error> {
        let ret = unsafe { ffi::comedi_set_read_subdevice(self.ptr, subdevice) };
        if ret < 0 {
            perror!("comedi_set_read_subdevice");
        }
        Ok(())
    }
    pub fn read_sampl<T>(&mut self, buf: &mut [T]) -> Result<usize, std::io::Error>
    where
        T: SamplType,
    {
        let ret = unsafe {
            libc::read(
                ffi::comedi_fileno(self.ptr),
                buf.as_mut_ptr().cast(),
                buf.len() * std::mem::size_of::<T>(),
            )
        };
        if ret < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(TryInto::<usize>::try_into(ret).unwrap() / std::mem::size_of::<T>())
        }
    }
}

impl Read for Comedi {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let ret = unsafe {
            libc::read(
                ffi::comedi_fileno(self.ptr),
                buf.as_mut_ptr().cast(),
                buf.len(),
            )
        };
        if ret < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret.try_into().unwrap())
        }
    }
}

pub fn to_phys(data: LSampl, range: &Range, maxdata: LSampl) -> Result<c_double, Error> {
    let phys = unsafe { ffi::comedi_to_phys(data, range.ptr.get(), maxdata) };
    if phys == c_double::NAN {
        perror!("comedi_to_phys");
    }
    Ok(phys)
}

fn cr_pack(chan: c_uint, rng: c_uint, aref: ARef) -> c_uint {
    ((aref.repr() & 0x3) << 24) | ((rng & 0xff) << 16) | (chan & 0xff)
}

fn cr_unpack(cr: c_uint) -> (c_uint, c_uint, ARef) {
    (
        cr & 0xff,
        (cr >> 16) & 0xff,
        ARef::from_repr((cr >> 24) & 0x3).unwrap(),
    )
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
