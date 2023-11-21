use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::Write;

fn copy_dir_override(src: &Path, dest: &Path) {
  println!("copy {} -> {}", src.display(), dest.display());
  let mut opts = fs_extra::dir::CopyOptions::new();
  opts.overwrite = true;
  fs_extra::dir::copy(src, dest, &opts).expect("failed to copy files");
}

fn build_luajit(luajit_dir: &Path, out_dir: &Path) {
  let luajit_out_dir = out_dir.join(luajit_dir.file_name().unwrap());
  let src_dir = luajit_out_dir.join("src");

  let target = env::var("TARGET").unwrap();
  let mut is_msvc = target.contains("msvc");

  let target_file: &str;
  let libname: &str;
  if is_msvc {
    target_file = "libluajit.a";
    libname = "lua51";
  } else {
    target_file = "libluajit.a";
    libname = "luajit";
  }
  println!("cargo:rustc-link-search=native={}", src_dir.display());
  println!("cargo:rustc-link-lib=static={}", libname);

  let out_lib = src_dir.join(target_file);
  if out_lib.exists() {
    return;
  }
  println!("building luajit.a");
  copy_dir_override(&luajit_dir, out_dir);
  let cc = cc::Build::new().get_compiler();
  if is_msvc {
    let mut cmd = Command::new(src_dir.join("msvcbuild.bat"));
    cmd.arg("static");
    cmd.current_dir(&src_dir);
    cmd.stderr(Stdio::inherit());
    configure_command_env(&mut cmd, cc);
    let r = cmd.status().expect("failed to call msvcbuild.bat");
    if !r.success() {
      panic!("msvcbuild.bat returned non zero error code: {}", r);
    }
  } else {
    let mut cmd = Command::new("make");
    cmd.arg(target_file);
    cmd.current_dir(&src_dir);
    cmd.stderr(Stdio::inherit());
    configure_command_env(&mut cmd, cc);
    let r = cmd.status().expect("failed to call make");
    if !r.success() {
      panic!("make returned non zero error code: {}", r);
    }
  }
}

fn configure_command_env(
  cmd: &mut Command,
  cc: cc::Tool,
) -> Result<std::process::ExitStatus, std::io::Error> {
  cmd.env("CC", cc.cc_env());
  for (name, value) in cc.env() {
    cmd.env(name, value);
  }
  cmd.status()
}

fn generate_bindings(src_dir: &Path, out_file: &Path) {
  let bindings = bindgen::builder()
    .header(src_dir.join("src/lua.hpp").to_str().unwrap())
    .default_enum_style(bindgen::EnumVariation::Consts)
    .whitelist_var("^LUA_.*")
    .whitelist_var("^LUAJIT_.*")
    .whitelist_type("^luaL?_.*")
    .whitelist_function("^luaL?_.*")
    .whitelist_function("^luaJIT_.*")
    .constified_enum(".*")
    .use_core()
    .ctypes_prefix("libc")
    .disable_untagged_union()
    .generate()
    .expect("Unable to generate bindings");

  let bindings = bindings.to_string();

  // replace u32 constants by c_int
  let re = regex::Regex::new(r"(?m)^(pub const.*:\s*)([iu]32|_bindgen_ty_\d+)(\s*=)").unwrap();
  let bindings = re.replace_all(&bindings, "${1}c_int${3}");
  // remove _bindgen_ty_#
  let re = regex::Regex::new(r#"(?m)^pub const _bindgen_ty_([^;]|\n);\r?\n"#).unwrap();
  let bindings = re.replace_all(&bindings, "");
  // remove string constants
  let re = regex::Regex::new(r#"(?m)^pub const.*=[\s\r\n]*b?"[^"]*"\s*;\s*\r?\n"#).unwrap();
  let bindings = re.replace_all(&bindings, "");
  // libc
  let re = regex::Regex::new(r"\blibc::c_").unwrap();
  let bindings = re.replace_all(&bindings, "c_");

  let mut f = std::fs::File::create(out_file).expect("unable to create bindings file");
  f.write(
    r#"
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_void,c_int,c_uint,c_char};

"#
    .as_bytes(),
  )
  .unwrap();
  f.write(bindings.as_bytes()).unwrap();
}

fn main() {
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  let luajit_dir = manifest_dir.join("luajit-2.0");

  if let Err(e) = pkg_config::Config::new()
    .atleast_version("2.0.0")
    .statik(true)
    .probe("luajit")
  {
    println!("system-library not found: {}", e);
    println!("building!");
    build_luajit(&luajit_dir, &out_dir);
  }

  let bindings_file = manifest_dir.join("src/ffi.rs");
  println!("cargo:rerun-if-changed={}", bindings_file.display());
  if !bindings_file.exists() {
    generate_bindings(&luajit_dir, &bindings_file);
  }
}
