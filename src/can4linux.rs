#[repr(C)]
pub struct Data {
    pub x: cty::c_int,
    pub y: cty::c_int,
}

pub extern "C" fn cool_function(
    i: cty::c_int,
    c: cty::c_char,
    cs: *mut CoolStruct
);
