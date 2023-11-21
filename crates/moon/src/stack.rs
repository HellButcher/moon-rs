use libc::c_int;
use crate::ffi;
use crate::State;

pub struct Void;

pub struct StackGuard<'l, V=Void> {
  #[allow(non_snake_case)]
  L: *mut ffi::lua_State,
  size: c_int,
  p: std::marker::PhantomData<&'l State>,
}

impl<'l> StackGuard<'l> {
  #[inline]
  unsafe fn assert_one_and_forget(self) -> c_int {
    assert_eq!(self.size, 1);
    self.forget()
  }

  #[inline]
  unsafe fn forget(mut self) -> c_int {
    let size = self.size;
    self.size = 0;
    size
  }
}

impl<'l> Drop for StackGuard<'l> {
  #[inline]
  fn drop(&mut self) {
    if self.size != 0 {
      unsafe {
        ffi::lua_pop(self.L, self.size);
      }
    }
  }
}
