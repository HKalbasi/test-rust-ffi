use std::ffi::c_void;

#[unsafe(no_mangle)]
pub extern "C" fn new_vec() -> *mut c_void {
    let r: Box<Vec<u64>> = Box::new(vec![]);
    Box::into_raw(r) as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "C" fn free_vec(v: *mut c_void) {
    unsafe {
        _ = Box::from_raw(v as *mut Vec<u64>);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn push_to_vec(v: *mut c_void, i: u64) {
    let v = unsafe { &mut *(v as *mut Vec<u64>) };
    v.push(i);
}
