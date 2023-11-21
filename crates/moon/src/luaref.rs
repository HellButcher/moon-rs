use std::fmt;
use libc::c_int;
use crate::ffi;
use crate::State;
use crate::stack::StackGuard;

struct RefRegistry {
  free_list: Vec<c_int>,
}

fn get_ref_registry(state: &mut State) -> StackGuard<'l, &'l RefRegistry> {
  
}

pub struct LuaRef<'l> {
  state: &'l State,
  index: c_int,
}

impl<'l> fmt::Debug for LuaRef<'l> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "LuaRef({})", self.index)
  }
}

impl<'l> Clone for LuaRef<'l> {
  fn clone(&self) -> Self {
      self.state.clone_ref(self)
  }
}

impl<'l> Drop for LuaRef<'l> {
  fn drop(&mut self) {
      self.state.drop_ref(self)
  }
}
