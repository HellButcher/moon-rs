#![allow(non_snake_case)]

use moon_luajit_sys::*;
use libc::c_char;

const SAMPLE_1 : &[u8] = br#"
-- Receives a table, returns the sum of its components.
x = 0
for i = 1, #foo do
  x = x + foo[i]
end
return x
"#;

#[test]
fn it_works() {
  unsafe {
    let L = luaL_newstate();
    assert!(!L.is_null());

    let status = luaJIT_setmode(L, 0, LUAJIT_MODE_ENGINE | LUAJIT_MODE_ON);
    assert_eq!(status, 0);

    luaL_openlibs(L);

    let status = luaL_loadslice(L, SAMPLE_1);
    assert_eq!(status, 0);

    lua_newtable(L);
    for i in 1i32..6 {
      lua_pushnumber(L, i as f64);
      lua_pushnumber(L, (i * 2) as f64);
      lua_rawset(L, -3);
    }

    const FOO: &[u8] = b"foo\0";
    lua_setglobal(L, FOO.as_ptr() as *const c_char);

    let status = lua_pcall(L, 0, LUA_MULTRET, 0);
    assert_eq!(status, 0);

    let sum = lua_tonumber(L, -1);
    assert_eq!(sum, 30 as f64);

    lua_pop(L, 1);
    lua_close(L);
  }
}
