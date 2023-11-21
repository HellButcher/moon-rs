#![allow(non_snake_case)]

use libc::{c_int, c_char, c_long};
use ::core::ptr::null_mut;
use crate::ffi::*;

#[inline]
pub fn lua_upvalueindex(i: c_int) -> c_int {
  LUA_GLOBALSINDEX-i
}

#[inline]
pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
  lua_settop(L, -n-1)
}

#[inline]
pub unsafe fn lua_newtable(L: *mut lua_State) {
  lua_createtable(L, 0, 0)
}

#[inline]
pub unsafe fn lua_register(L: *mut lua_State, n: *const c_char, f: lua_CFunction) {
  lua_pushcfunction(L, f);
  lua_setglobal(L, n)
}

#[inline]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
  lua_pushcclosure(L, f, 0)
}

#[inline]
pub unsafe fn lua_strlen(L: *mut lua_State, i: c_int) -> usize {
  lua_objlen(L, i)
}

#[inline]
pub unsafe fn lua_isfunction(L: *mut lua_State, n: c_int) -> bool {
  lua_type(L, n) == LUA_TFUNCTION
}

#[inline]
pub unsafe fn lua_istable(L: *mut lua_State, n: c_int) -> bool {
  lua_type(L, n) == LUA_TTABLE
}

#[inline]
pub unsafe fn lua_islightuserdata(L: *mut lua_State, n: c_int) -> bool {
  lua_type(L, n) == LUA_TLIGHTUSERDATA
}

#[inline]
pub unsafe fn lua_isnil(L: *mut lua_State, n: c_int) -> bool {
  lua_type(L, n) == LUA_TNIL
}

#[inline]
pub unsafe fn lua_isboolean(L: *mut lua_State, n: c_int) -> bool {
  lua_type(L, n) == LUA_TBOOLEAN
}

#[inline]
pub unsafe fn lua_isthread(L: *mut lua_State, n: c_int) -> bool {
  lua_type(L, n) == LUA_TTHREAD
}

#[inline]
pub unsafe fn lua_isnone(L: *mut lua_State, n: c_int) -> bool {
  lua_type(L, n) == LUA_TNONE
}

#[inline]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, n: c_int) -> bool {
  lua_type(L, n) <= 0
}

#[inline]
pub unsafe fn lua_pushslice<D: AsRef<[u8]> + ?Sized>(L: *mut lua_State, s: &D) {
  let s = s.as_ref();
  lua_pushlstring(L, s.as_ptr() as *const c_char, s.len())
}

pub use lua_pushslice as lua_pushliteral;

#[inline]
pub unsafe fn lua_setglobal(L: *mut lua_State, s: *const c_char) {
  lua_setfield(L, LUA_GLOBALSINDEX, s)
}

#[inline]
pub unsafe fn lua_getglobal(L: *mut lua_State, s: *const c_char) {
  lua_getfield(L, LUA_GLOBALSINDEX, s)
}

#[inline]
pub unsafe fn lua_tostring(L: *mut lua_State, i: c_int) -> *const c_char {
  lua_tolstring(L, i, null_mut())
}


/*
** compatibility macros and functions
*/

pub use super::luaL_newstate as lua_open;

#[inline]
pub unsafe fn lua_getregistry(L: *mut lua_State) {
  lua_pushvalue(L, LUA_REGISTRYINDEX)
}

#[inline]
pub unsafe fn lua_getgccount(L: *mut lua_State) -> c_int {
  lua_gc(L, LUA_GCCOUNT, 0)
}

pub use super::lua_Reader as lua_Chunkreader;
pub use super::lua_Writer as lua_Chunkwriter;

#[inline]
pub unsafe fn luaL_getn(L: *mut lua_State, i: c_int) -> c_int {
  lua_objlen(L, i) as c_int
}

#[inline]
pub fn luaL_setn(_L: *mut lua_State, _i: c_int, _j: c_int) {
  // NOOP
}

#[inline]
pub unsafe fn luaL_argcheck(L: *mut lua_State, cond: c_int, numarg: c_int,
  extramsg: *const c_char,
) -> c_int {
  if cond != 0 || luaL_argerror(L, numarg, extramsg) != 0 {1} else {0}
}

#[inline]
pub unsafe fn luaL_checkstring(L: *mut lua_State, n: c_int) -> *const c_char {
  luaL_checklstring(L, n, null_mut())
}

#[inline]
pub unsafe fn luaL_optstring(L: *mut lua_State, n: c_int, d: *const c_char) -> *const c_char {
  luaL_optlstring(L, n, d, null_mut())
}

#[inline]
pub unsafe fn luaL_checkint(L: *mut lua_State, n: c_int) -> c_int {
  luaL_checkinteger(L, n) as c_int
}

#[inline]
pub unsafe fn luaL_optint(L: *mut lua_State, n: c_int, d: c_int) -> c_int {
  luaL_optinteger(L, n, d as lua_Integer) as c_int
}

#[inline]
pub unsafe fn luaL_checklong(L: *mut lua_State, n: c_int) -> c_long {
  luaL_checkinteger(L, n) as c_long
}

#[inline]
pub unsafe fn luaL_optlong(L: *mut lua_State, n: c_int, d: c_long) -> c_long {
  luaL_optinteger(L, n, d as lua_Integer) as c_long
}

#[inline]
pub unsafe fn luaL_typename(L: *mut lua_State, i: c_int) -> *const c_char {
  lua_typename(L, lua_type(L,i))
}

#[inline]
pub unsafe fn luaL_dofile(L: *mut lua_State, _fn: *const c_char) -> c_int {
  if luaL_loadfile(L, _fn) != 0 || lua_pcall(L, 0, LUA_MULTRET, 0) != 0 {1} else {0}
}

#[inline]
pub unsafe fn luaL_dostring(L: *mut lua_State, s: *const c_char) -> c_int {
  if luaL_loadstring(L, s) != 0 || lua_pcall(L, 0, LUA_MULTRET, 0) != 0 {1} else {0}
}

#[inline]
pub unsafe fn luaL_loadslice<D: AsRef<[u8]> + ?Sized>(L: *mut lua_State, s: &D) -> c_int {
  let s = s.as_ref();
  luaL_loadbuffer(L, s.as_ptr() as *const c_char, s.len(), b"slice\0".as_ptr() as *const c_char)
}

#[inline]
pub unsafe fn luaL_doslice<D: AsRef<[u8]> + ?Sized>(L: *mut lua_State, s: &D) -> c_int {
  if luaL_loadslice(L, s) != 0 || lua_pcall(L, 0, LUA_MULTRET, 0) != 0 {1} else {0}
}

#[inline]
pub unsafe fn luaL_getmetatable(L: *mut lua_State, n: *const c_char) {
  lua_getfield(L, LUA_REGISTRYINDEX, n)
}

#[inline]
pub unsafe fn luaL_opt<F,R>(L: *mut lua_State, f: F, n: c_int, d: R) -> R where F: FnOnce(*mut lua_State, c_int) -> R {
  if lua_isnoneornil(L, n) {
    d
  } else {
    f(L,n)
  }
}

#[inline]
pub unsafe fn luaL_addchar(B: *mut luaL_Buffer, c: char) {
  let p = (*B).p;
  if p as *const _ >= (*B).buffer.as_ptr().add((*B).buffer.len()) {
    luaL_prepbuffer(B);
  }
  *p = c as c_char;
  (*B).p = p.add(1);
}

/* compatibility only */
pub use luaL_addchar as luaL_putchar;

#[inline]
pub unsafe fn luaL_addsize(B: *mut luaL_Buffer, n: isize) {
  (*B).p = (*B).p.offset(n);
}

#[inline]
pub unsafe fn lua_ref(L: *mut lua_State, lock: bool) -> c_int {
  if lock {
    luaL_ref(L, LUA_REGISTRYINDEX)
  } else {
    lua_pushslice(L, b"unlocked references are obsolete");
    lua_error(L);
    0
  }
}


#[inline]
pub unsafe fn lua_unref(L: *mut lua_State, ref_: c_int) {
  luaL_unref(L, LUA_REGISTRYINDEX, ref_)
}

#[inline]
pub unsafe fn lua_getref(L: *mut lua_State, ref_: c_int) {
  lua_rawgeti(L, LUA_REGISTRYINDEX, ref_)
}

pub use super::luaL_Reg as luaL_reg;
