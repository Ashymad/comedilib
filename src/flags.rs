use comedilib_sys as ffi;
use std::os::raw::c_uint;

pub trait Bitfield
where
    Self: Sized,
{
    fn is_set(&self, other: Self) -> bool;
    fn set(&mut self, other: Self);
}

pub struct SDF {
    value: c_uint,
}

impl Bitfield for SDF {
    fn is_set(&self, other: Self) -> bool {
        self.value & other.value != 0
    }
    fn set(&mut self, other: Self) {
        self.value |= other.value;
    }
}

impl SDF {
    pub fn new(value: c_uint) -> SDF {
        SDF { value }
    }

    pub const BUSY: SDF = SDF {
        value: ffi::SDF_BUSY,
    };
    pub const BUSY_OWNER: SDF = SDF {
        value: ffi::SDF_BUSY_OWNER,
    };
    pub const LOCKED: SDF = SDF {
        value: ffi::SDF_LOCKED,
    };
    pub const LOCK_OWNER: SDF = SDF {
        value: ffi::SDF_LOCK_OWNER,
    };
    pub const MAXDATA: SDF = SDF {
        value: ffi::SDF_MAXDATA,
    };
    pub const FLAGS: SDF = SDF {
        value: ffi::SDF_FLAGS,
    };
    pub const RANGETYPE: SDF = SDF {
        value: ffi::SDF_RANGETYPE,
    };
    pub const CMD: SDF = SDF {
        value: ffi::SDF_CMD,
    };
    pub const SOFT_CALIBRATED: SDF = SDF {
        value: ffi::SDF_SOFT_CALIBRATED,
    };
    pub const READABLE: SDF = SDF {
        value: ffi::SDF_READABLE,
    };
    pub const WRITABLE: SDF = SDF {
        value: ffi::SDF_WRITABLE,
    };
    pub const INTERNAL: SDF = SDF {
        value: ffi::SDF_INTERNAL,
    };
    pub const GROUND: SDF = SDF {
        value: ffi::SDF_GROUND,
    };
    pub const COMMON: SDF = SDF {
        value: ffi::SDF_COMMON,
    };
    pub const DIFF: SDF = SDF {
        value: ffi::SDF_DIFF,
    };
    pub const OTHER: SDF = SDF {
        value: ffi::SDF_OTHER,
    };
    pub const DITHER: SDF = SDF {
        value: ffi::SDF_DITHER,
    };
    pub const DEGLITCH: SDF = SDF {
        value: ffi::SDF_DEGLITCH,
    };
    pub const RUNNING: SDF = SDF {
        value: ffi::SDF_RUNNING,
    };
    pub const LSAMPL: SDF = SDF {
        value: ffi::SDF_LSAMPL,
    };
    pub const PACKED: SDF = SDF {
        value: ffi::SDF_PACKED,
    };
}
