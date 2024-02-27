use crate::joy::{MID_VAL, X_MAX, X_MID, X_MIN, Y_MAX, Y_MID, Y_MIN};

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn joy_map(val: u16, in_min: u16, in_max: u16, out_min: u16, out_max: u16) -> u16 {
    let a: u32 = (val - in_min).into();
    let b: u32 = (out_max - out_min).into();
    let c: u32 = (in_max - in_min).into();

    let d: u16 = (a * b / c).try_into().unwrap();
    d + out_min
}

#[must_use]
pub fn normalize_x(val: u16) -> u16 {
    if val < X_MID {
        joy_map(val, X_MIN, X_MID, 0, MID_VAL)
    } else {
        joy_map(val, X_MID, X_MAX, 2048, X_MAX)
    }
}

#[must_use]
pub fn normalize_y(val: u16) -> u16 {
    if val < Y_MID {
        joy_map(val, Y_MIN, Y_MID, 0, MID_VAL)
    } else {
        joy_map(val, Y_MID, Y_MAX, 2048, Y_MAX)
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn mirror_in_range(val: u64, min: u64, max: u64) -> u64 {
    max - (val - min)
}
