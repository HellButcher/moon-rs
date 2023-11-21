use libc::c_int;
use bstr::BStr;
use crate::ffi;
use crate::error::{LuaError::*,Result};
use crate::convert::to_bstring;

pub struct StateBuilder {

}

pub struct State {
  #[allow(non_snake_case)]
  pub(crate) L: *mut ffi::lua_State,
}

impl StateBuilder {
  pub fn new() -> StateBuilder {
    StateBuilder{}
  }

  #[inline]
  pub fn build(&self) -> Result<State> {
    unsafe {
      let l = ffi::luaL_newstate();
      if l.is_null() {
        Err(AllocationError(None))?
      }
      unsafe { ffi::lua_atpanic(l, Some(lua_panic_handler)) };
      Ok(State{
        L: l,
      })
    }
  }
}

impl Default for StateBuilder {
  #[inline]
  fn default() -> StateBuilder {
    StateBuilder::new()
  }
}

#[inline]
pub fn builder() -> StateBuilder{
  StateBuilder::new()
}

impl State {
  #[inline]
  pub fn builder() -> StateBuilder {
    StateBuilder::new()
  }

  #[inline]
  pub fn new() -> Result<State> {
    StateBuilder::new().build()
  }

  #[inline]
  pub unsafe fn from_raw(state: *mut ffi::lua_State) -> State {
    State{
      L: state,
    }
  }

  #[inline]
  pub unsafe fn into_raw(mut self) -> *mut ffi::lua_State {
    std::mem::replace(&mut self.L, std::ptr::null_mut())
  }
}

impl Drop for State {
  #[inline]
  fn drop(&mut self) {
    unsafe {
      ffi::lua_close(std::mem::replace(&mut self.L, std::ptr::null_mut()));
    }
  }
}


extern "C" fn lua_panic_handler(lua: *mut ffi::lua_State) -> c_int {
  let err = unsafe { to_bstring(lua, -1) };
  let err: &BStr = err.as_ref().map_or("Unknown".as_ref(), AsRef::as_ref);
  panic!("PANIC: unprotected error in call to Lua API ({})\n", err);
}
