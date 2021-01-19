use comedilib_sys as ffi;
use enum_repr::EnumRepr;
use std::os::raw::{c_int, c_uint};

pub type LSampl = ffi::lsampl_t;
pub type Sampl = ffi::sampl_t;

pub trait SamplType {}
impl SamplType for LSampl {}
impl SamplType for Sampl {}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ARef {
    Ground = ffi::AREF_GROUND,
    Common = ffi::AREF_COMMON,
    Diff = ffi::AREF_DIFF,
    Other = ffi::AREF_OTHER,
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OORBehavior {
    NaN = ffi::comedi_oor_behavior_COMEDI_OOR_NAN,
    Number = ffi::comedi_oor_behavior_COMEDI_OOR_NUMBER,
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Unit {
    Volt = ffi::UNIT_volt,
    MiliAmper = ffi::UNIT_mA,
    None = ffi::UNIT_none,
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StartTrigger {
    Now = ffi::TRIG_NOW,
    Follow = ffi::TRIG_FOLLOW,
    Ext = ffi::TRIG_EXT,
    Int = ffi::TRIG_INT,
    Other = ffi::TRIG_OTHER,
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ScanBeginTrigger {
    Timer = ffi::TRIG_TIMER,
    Follow = ffi::TRIG_FOLLOW,
    Ext = ffi::TRIG_EXT,
    Other = ffi::TRIG_OTHER,
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ConvertTrigger {
    Now = ffi::TRIG_NOW,
    Timer = ffi::TRIG_TIMER,
    Ext = ffi::TRIG_EXT,
    Other = ffi::TRIG_OTHER,
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ScanEndTrigger {
    Count = ffi::TRIG_COUNT,
    Other = ffi::TRIG_OTHER,
}

#[EnumRepr(type = "c_uint")]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StopTrigger {
    Count = ffi::TRIG_COUNT,
    None = ffi::TRIG_NONE,
    Other = ffi::TRIG_OTHER,
}

#[EnumRepr(type = "c_int")]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CommandTestResult {
    Ok = 0,
    TriggerUnsupported = 1,
    TriggerCombinationUnsupported = 2,
    ArgumentOutOfRange = 3,
    ArgumentRequiredAdjustment = 4,
    ChanlistUnsupported = 5,
}
