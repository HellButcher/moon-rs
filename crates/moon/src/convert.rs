use crate::ffi::*;
use libc::{c_int,c_char,c_void,size_t};
use bstr::BString;
use std::{mem,ptr,slice};

#[inline]
pub unsafe fn to_bstr<'l>(state: *mut lua_State, i: c_int) -> Option<&'l [u8]> {
  let mut len: size_t = 0;
  let s = ffi::lua_tolstring(state, i, &mut len);
  if s.is_null() {
    None
  } else {
    Some(slice::from_raw_parts(s as *const u8, len))
  }
}
#[inline]
pub unsafe fn to_bstring(state: *mut lua_State, i: c_int) -> Option<BString> {
  to_bstr(state, i).map(Into::into)
}


#[inline]
pub unsafe fn to_bool(state: *mut lua_State, i: c_int) -> bool {
  ffi::lua_toboolean(state, i) != 0
}

#[inline]
pub unsafe fn to_userdata_ptr<T>(state: *mut lua_State, i: c_int) -> *mut T {
  ffi::lua_touserdata(state, i) as *mut T
}

#[inline]
pub unsafe fn get_userdata_unchecked<T: Clone>(state: *mut lua_State, i: c_int) -> Option<T> {
  let p = to_userdata_ptr(state, i) as *const T;
  if p.is_null() {
    return None;
  }
  return Some((*p).clone())
}
