use libc::c_int;
use bstr::BString;
use std::fmt;
use crate::ffi;

#[derive(Debug)]
pub enum Error {
  LuaError(LuaError),
  IoError(std::io::Error),
}

#[derive(Debug)]
pub enum LuaError {
  AllocationError(Option<BString>),
  SyntaxError(Option<BString>),
  RuntimeError(Option<BString>),
  FileError(Option<BString>),
  NestedError(Option<BString>),
  OtherError(c_int, Option<BString>),
}


impl Error {
  #[inline]
  pub fn status(&self) -> c_int {
    match *self {
      Error::LuaError(ref e) => e.status(),
      Error::IoError(_) => ffi::LUA_ERRFILE,
    }
  }
}
impl LuaError {
  #[inline]
  pub fn status(&self) -> c_int {
    match *self {
      LuaError::AllocationError(_) => ffi::LUA_ERRMEM,
      LuaError::SyntaxError(_) => ffi::LUA_ERRSYNTAX,
      LuaError::RuntimeError(_) => ffi::LUA_ERRRUN,
      LuaError::FileError(_) => ffi::LUA_ERRFILE,
      LuaError::NestedError(_) => ffi::LUA_ERRERR,
      LuaError::OtherError(s, _) => s,
    }
  }
}

impl From<std::io::Error> for Error {
  #[inline]
  fn from(e: std::io::Error) -> Error {
    Error::IoError(e)
  }
}

impl From<LuaError> for Error {
  #[inline]
  fn from(e: LuaError) -> Error {
    Error::LuaError(e)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::LuaError(ref e) => fmt::Display::fmt(e, f),
      Error::IoError(ref e) => write!(f, "IO Error: {}", e),
    }
  }
}

impl fmt::Display for LuaError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let e = match *self {
      LuaError::AllocationError(ref e) => {
        write!(f, "Allocation Error")?;
        e
      },
      LuaError::SyntaxError(ref e) => {
        write!(f, "Syntax Error")?;
        e
      },
      LuaError::RuntimeError(ref e) => {
        write!(f, "Runtime Error")?;
        e
      },
      LuaError::FileError(ref e) => {
        write!(f, "File Error")?;
        e
      },
      LuaError::NestedError(ref e) => {
        write!(f, "Nested Error")?;
        e
      },
      LuaError::OtherError(s, ref e) => {
        write!(f, "Unknown Lua Error {}", s)?;
        e
      },
    };
    if let Some(ref e) = *e {
      write!(f, ": {}", e)?;
    }
    Ok(())
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::LuaError(ref e) => Some(e),
      Error::IoError(ref e) => Some(e),
    }
  }
}

impl std::error::Error for LuaError {
}

pub type Result<S,E=Error> = ::std::result::Result<S,E>;

