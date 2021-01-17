use comedilib_sys as ffi;
use enum_repr::EnumRepr;
use std::os::raw::c_uint;

pub type LSampl = ffi::lsampl_t;
pub type Sampl = ffi::lsampl_t;

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone)]
pub enum ARef {
    Ground = ffi::AREF_GROUND,
    Common = ffi::AREF_COMMON,
    Diff = ffi::AREF_DIFF,
    Other = ffi::AREF_OTHER,
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone)]
pub enum OORBehavior {
    NaN = ffi::comedi_oor_behavior_COMEDI_OOR_NAN,
    Number = ffi::comedi_oor_behavior_COMEDI_OOR_NUMBER,
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone)]
pub enum Unit {
    Volt = ffi::UNIT_volt,
    MiliAmper = ffi::UNIT_mA,
    None = ffi::UNIT_none,
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone)]
pub enum StartTrigger {
    Now = ffi::TRIG_NOW,
    Follow = ffi::TRIG_FOLLOW,
    Ext = ffi::TRIG_EXT,
    Int = ffi::TRIG_INT,
    Other = ffi::TRIG_OTHER
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone)]
pub enum ScanBeginTrigger {
    Timer = ffi::TRIG_TIMER,
    Follow = ffi::TRIG_FOLLOW,
    Ext = ffi::TRIG_EXT,
    Other = ffi::TRIG_OTHER
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone)]
pub enum ConvertTrigger {
    Now = ffi::TRIG_NOW,
    Timer = ffi::TRIG_TIMER,
    Ext = ffi::TRIG_EXT,
    Other = ffi::TRIG_OTHER
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone)]
pub enum ScanEndTrigger {
    Count = ffi::TRIG_COUNT,
    Other = ffi::TRIG_OTHER
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone)]
pub enum StopTrigger {
    Count = ffi::TRIG_COUNT,
    None = ffi::TRIG_NONE,
    Other = ffi::TRIG_OTHER
}
